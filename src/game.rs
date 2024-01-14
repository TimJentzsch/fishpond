use bevy::prelude::*;
use pleco::{Board, Player};

use crate::engine::StartEngine;

#[derive(Debug, Default, Component)]
pub struct Game;

#[derive(Debug, Component, Clone, Copy)]
pub struct GameRef {
    pub game_id: Entity,
    pub player: Player,
}

#[derive(Debug, Default, Component, Deref, DerefMut)]
pub struct GameBoard(Board);

#[derive(Debug, Event)]
pub struct CreateGame;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_game_creation);
    }
}

fn handle_game_creation(
    mut create_game_event: EventReader<CreateGame>,
    mut commands: Commands,
    mut start_engine_event: EventWriter<StartEngine>,
) {
    for _ in create_game_event.read() {
        let game_id = commands.spawn((Game, GameBoard::default())).id();

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
