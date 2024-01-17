use bevy::prelude::*;
use pleco::{Board, Player};

use crate::engine::{EngineInitialized, SearchMove, SearchResult, StartEngine};

#[derive(Debug, Default, Component)]
pub struct Game;

#[derive(Debug, Component, Clone, Copy, PartialEq)]
pub struct GameRef {
    pub game_id: Entity,
    pub player: Player,
}

#[derive(Debug, Component)]
pub enum GameState {
    PlayerInitialization { white: bool, black: bool },
    WaitingForPlayer { player: Player },
    Finished,
}

#[derive(Debug, Default, Component, Deref, DerefMut, Clone)]
pub struct GameBoard(Board);

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
                player: Player::White,
            },
            path: "stockfish".to_string(),
        });
        start_engine_event.send(StartEngine {
            game_ref: GameRef {
                game_id,
                player: Player::Black,
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
                let new_white = white || engine_initialized.game_ref.player == Player::White;
                let new_black = black || engine_initialized.game_ref.player == Player::Black;

                if new_white && new_black {
                    *game_state = GameState::WaitingForPlayer {
                        player: Player::White,
                    };

                    // White moves first
                    search_move_event.send(SearchMove {
                        game_ref: GameRef {
                            game_id,
                            player: Player::White,
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

            if !game_board.apply_uci_move(&search_result.uci_move) {
                println!("Invalid UCI move {}", search_result.uci_move);
                continue;
            }

            println!("Played {} -> {}", search_result.uci_move, game_board.fen());

            if game_board.checkmate() {
                *game_state = GameState::Finished;

                println!("Game over, {:?} won!", game_board.turn().other_player());
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
