use bevy::prelude::*;
use shakmaty::{fen::Fen, Chess, Color, Move, Outcome, Position};

use crate::engine::{EngineInitialized, SearchMove, SearchResult, StartEngine};

mod game;

#[derive(Debug, Default, Component)]
pub struct Game;

#[derive(Debug, Component, Clone, Copy, PartialEq)]
pub struct GameRef {
    pub game_id: Entity,
    pub player: Color,
}

#[derive(Debug, Component)]
pub enum GameState {
    PlayerInitialization { white: bool, black: bool },
    WaitingForPlayer { player: Color },
    Finished,
}

#[derive(Debug, Default, Component, Deref, DerefMut, Clone)]
pub struct GameBoard(Chess);

impl Position for GameBoard {
    fn board(&self) -> &shakmaty::Board {
        self.0.board()
    }

    fn promoted(&self) -> shakmaty::Bitboard {
        self.0.promoted()
    }

    fn pockets(&self) -> Option<&shakmaty::ByColor<shakmaty::ByRole<u8>>> {
        self.0.pockets()
    }

    fn turn(&self) -> Color {
        self.0.turn()
    }

    fn castles(&self) -> &shakmaty::Castles {
        self.0.castles()
    }

    fn maybe_ep_square(&self) -> Option<shakmaty::Square> {
        self.0.maybe_ep_square()
    }

    fn remaining_checks(&self) -> Option<&shakmaty::ByColor<shakmaty::RemainingChecks>> {
        self.0.remaining_checks()
    }

    fn halfmoves(&self) -> u32 {
        self.0.halfmoves()
    }

    fn fullmoves(&self) -> std::num::NonZeroU32 {
        self.0.fullmoves()
    }

    fn into_setup(self, mode: shakmaty::EnPassantMode) -> shakmaty::Setup {
        self.0.into_setup(mode)
    }

    fn legal_moves(&self) -> shakmaty::MoveList {
        self.0.legal_moves()
    }

    fn is_variant_end(&self) -> bool {
        self.0.is_variant_end()
    }

    fn has_insufficient_material(&self, color: Color) -> bool {
        self.0.has_insufficient_material(color)
    }

    fn variant_outcome(&self) -> Option<Outcome> {
        self.0.variant_outcome()
    }

    fn play_unchecked(&mut self, m: &Move) {
        self.0.play_unchecked(m)
    }
}

#[derive(Debug, Event)]
pub struct CreateGame;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CreateGame>().add_systems(
            Update,
            (
                handle_game_creation,
                handle_engine_startup_engine_initialization,
                handle_engine_search_result,
            ),
        );
    }
}

fn handle_game_creation(
    mut create_game_event: EventReader<CreateGame>,
    mut commands: Commands,
    mut start_engine_event: EventWriter<StartEngine>,
) {
    for _ in create_game_event.read() {
        let game_id = commands
            .spawn((
                Game,
                GameBoard::default(),
                GameState::PlayerInitialization {
                    white: false,
                    black: false,
                },
            ))
            .id();

        // Add players
        start_engine_event.send(StartEngine {
            game_ref: GameRef {
                game_id,
                player: Color::White,
            },
            path: "stockfish".to_string(),
        });
        start_engine_event.send(StartEngine {
            game_ref: GameRef {
                game_id,
                player: Color::Black,
            },
            path: "stockfish".to_string(),
        });
    }
}

fn handle_engine_startup_engine_initialization(
    mut engine_initialized_event: EventReader<EngineInitialized>,
    mut game_query: Query<(Entity, &mut GameState, &GameBoard)>,
    mut search_move_event: EventWriter<SearchMove>,
) {
    for engine_initialized in engine_initialized_event.read() {
        if let Ok((game_id, mut game_state, game_board)) =
            game_query.get_mut(engine_initialized.game_ref.game_id)
        {
            if let GameState::PlayerInitialization { white, black } = *game_state {
                let new_white = white || engine_initialized.game_ref.player == Color::White;
                let new_black = black || engine_initialized.game_ref.player == Color::Black;

                if new_white && new_black {
                    *game_state = GameState::WaitingForPlayer {
                        player: Color::White,
                    };

                    // White moves first
                    search_move_event.send(SearchMove {
                        game_ref: GameRef {
                            game_id,
                            player: Color::White,
                        },
                        game_board: game_board.clone(),
                    });
                } else {
                    *game_state = GameState::PlayerInitialization {
                        white: new_white,
                        black: new_black,
                    };
                }
            }
        }
    }
}

fn handle_engine_search_result(
    mut search_result_event: EventReader<SearchResult>,
    mut game_query: Query<(Entity, &mut GameState, &mut GameBoard)>,
    mut search_move_event: EventWriter<SearchMove>,
) {
    for search_result in search_result_event.read() {
        if let Ok((game_id, mut game_state, mut game_board)) =
            game_query.get_mut(search_result.game_ref.game_id)
        {
            if !search_result.game_ref.player == game_board.turn() {
                println!("Wrong player");
                continue;
            }

            let Ok(r#move) = search_result.uci_move.to_move(&*game_board) else {
                println!("Invalid UCI move");
                continue;
            };

            // Move is already validated when parsing UCI
            game_board.play_unchecked(&r#move);

            println!(
                "Played {} -> {}",
                search_result.uci_move,
                Fen::from_position(game_board.clone(), shakmaty::EnPassantMode::Legal)
            );

            if let Some(outcome) = game_board.outcome() {
                *game_state = GameState::Finished;

                match outcome {
                    Outcome::Decisive { winner } => println!("Game over, {winner} won!"),
                    Outcome::Draw => println!("Game over with a draw!"),
                };
            } else {
                // Next player's turn
                *game_state = GameState::WaitingForPlayer {
                    player: game_board.turn(),
                };

                search_move_event.send(SearchMove {
                    game_ref: GameRef {
                        game_id,
                        player: game_board.turn(),
                    },
                    game_board: game_board.clone(),
                });
            }
        }
    }
}
