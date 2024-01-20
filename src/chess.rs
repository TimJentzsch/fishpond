use bevy::prelude::*;
use fishpond_game::{DeclareDrawReason, Game, Outcome};
use shakmaty::{fen::Fen, Chess, Color, Position};

use crate::engine::{EngineInitialized, SearchMove, SearchResult, StartEngine};

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
                Game::from_start_position(Chess::default()),
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
    mut game_query: Query<(Entity, &mut GameState, &Game<Chess>)>,
    mut search_move_event: EventWriter<SearchMove>,
) {
    for engine_initialized in engine_initialized_event.read() {
        if let Ok((game_id, mut game_state, game)) =
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
                        game: game.clone(),
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
    mut game_query: Query<(Entity, &mut GameState, &mut Game<Chess>)>,
    mut search_move_event: EventWriter<SearchMove>,
) {
    for search_result in search_result_event.read() {
        if let Ok((game_id, mut game_state, mut game)) =
            game_query.get_mut(search_result.game_ref.game_id)
        {
            if !search_result.game_ref.player == game.turn() {
                println!("Wrong player");
                continue;
            }

            let Ok(r#move) = search_result.uci_move.to_move(&*game) else {
                println!(
                    "Invalid UCI move {} in position {}",
                    search_result.uci_move,
                    Fen::from_position(
                        game.current_position().clone(),
                        shakmaty::EnPassantMode::Legal
                    )
                );
                continue;
            };

            // Move is already validated when parsing UCI
            game.play_unchecked(&r#move);

            println!(
                "Played {} -> {}",
                search_result.uci_move,
                Fen::from_position(game.clone(), shakmaty::EnPassantMode::Legal)
            );

            // Check if the game should be declared as draw
            if let Some(reason) = game.can_declare_draw() {
                match reason {
                    DeclareDrawReason::Repetition {
                        count: repetitions,
                        claimed_by: _,
                    } => {
                        if repetitions >= 5 {
                            // Automatically declare draw on fivefold repetition
                            game.declare_draw()
                                .expect("Could not declare draw on fivefold repetition");
                        }
                    }
                    DeclareDrawReason::FiftyMoveRule { halfmoves } => {
                        if halfmoves >= 150 {
                            // Automatically declare draw on seventy-five-move rule
                            game.declare_draw()
                                .expect("Could not declare draw on seventy-five-move rule");
                        }
                    }
                }
            }

            // Check if the game is over
            if let Some(outcome) = game.game_outcome() {
                *game_state = GameState::Finished;

                match outcome {
                    Outcome::Decisive { winner, reason } => {
                        println!("GAME OVER | {winner} WON due to {reason:?}!")
                    }
                    Outcome::Draw { reason } => println!("GAME OVER | DRAW due to {reason:?}"),
                };
                return;
            }

            // Next player's turn
            *game_state = GameState::WaitingForPlayer {
                player: game.turn(),
            };

            search_move_event.send(SearchMove {
                game_ref: GameRef {
                    game_id,
                    player: game.turn(),
                },
                game: game.clone(),
            });
        }
    }
}
