use std::time::Duration;

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_local_commands::BevyLocalCommandsPlugin;
use engine::EnginePlugin;
use game::CreateGame;
use process_log::ProcessLogPlugin;

mod engine;
mod game;
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
        ))
        .add_systems(Startup, create_game)
        .run();
}

fn create_game(mut create_game_event: EventWriter<CreateGame>) {
    create_game_event.send(CreateGame);
}
