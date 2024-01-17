use bevy::prelude::*;
use pleco::{Board, Player};

use crate::engine::{EngineInitialized, StartEngine};

#[derive(Debug, Default, Component)]
pub struct Game;

#[derive(Debug, Component, Clone, Copy)]
pub struct GameRef {
    pub game_id: Entity,
    pub player: Player,
}

#[derive(Debug, Component)]
pub enum GameState {
    PlayerInitialization { white: bool, black: bool },
    WaitingForPlayer { player: Player },
}

#[derive(Debug, Default, Component, Deref, DerefMut)]
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
    mut game_query: Query<&mut GameState>,
) {
    for engine_initialized in engine_initialized_event.read() {
        if let Ok(mut game_state) = game_query.get_mut(engine_initialized.game_ref.game_id) {
            if let GameState::PlayerInitialization { white, black } = *game_state {
                let new_white = white || engine_initialized.game_ref.player == Player::White;
                let new_black = black || engine_initialized.game_ref.player == Player::Black;

                if new_white && new_black {
                    *game_state = GameState::WaitingForPlayer {
                        player: Player::White,
                    }
                    // TODO: Send first move command
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
