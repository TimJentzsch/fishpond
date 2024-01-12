use std::time::Duration;

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_local_commands::BevyLocalCommandsPlugin;
use engine::{EnginePlugin, StartEngine};
use process_log::ProcessLogPlugin;

mod engine;
mod process_log;

#[derive(Debug, Default, Component)]
struct Game;

#[derive(Debug, Component)]
struct GameRef(Entity);

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
        .add_systems(Startup, start_stockfish)
        .run();
}

fn start_stockfish(mut commands: Commands, mut start_engine_event: EventWriter<StartEngine>) {
    let game_id = commands.spawn(Game).id();
    start_engine_event.send(StartEngine {
        game_id,
        path: "stockfish".to_string(),
    });
}
