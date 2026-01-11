use std::time::Duration;

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_local_commands::BevyLocalCommandsPlugin;
use chess::{CreateGame, GamePlugin};
use engine::EnginePlugin;
use process_log::ProcessLogPlugin;

mod chess;
mod engine;
mod process_log;

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                // Limit to 30 FPS
                1.0 / 30.0,
            ))),
            BevyLocalCommandsPlugin,
            ProcessLogPlugin,
            EnginePlugin,
            GamePlugin,
        ))
        .add_systems(Startup, create_game)
        .run();
}

fn create_game(mut create_game_event: MessageWriter<CreateGame>) {
    create_game_event.write(CreateGame);
}
