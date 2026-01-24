use bevy::prelude::*;
use bevy_local_commands::BevyLocalCommandsPlugin;
use chess::{CreateGame, GamePlugin};
use engine::EnginePlugin;
use process_log::ProcessLogPlugin;

mod chess;
mod engine;
mod game;
mod process_log;

pub struct FishpondBackendPlugin;

impl Plugin for FishpondBackendPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            BevyLocalCommandsPlugin,
            ProcessLogPlugin,
            EnginePlugin,
            GamePlugin,
        ))
        .add_systems(Startup, create_game);
    }
}

fn create_game(mut create_game_event: MessageWriter<CreateGame>) {
    create_game_event.write(CreateGame);
}
