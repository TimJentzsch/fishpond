use std::time::Duration;

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_local_commands::{BevyLocalCommandsPlugin, ProcessError, ProcessOutput};
use engine::{EnginePlugin, StartEngine};

mod engine;

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
            EnginePlugin,
        ))
        .add_systems(Startup, start_stockfish)
        .add_systems(Update, (log_output, log_errors))
        .run();
}

fn start_stockfish(mut commands: Commands, mut start_engine_event: EventWriter<StartEngine>) {
    let game_id = commands.spawn(Game).id();
    start_engine_event.send(StartEngine {
        game_id,
        path: "stockfish".to_string(),
    });
}

fn log_output(mut output_event: EventReader<ProcessOutput>) {
    for output in output_event.read() {
        println!("{}", output.output.join("\n"));
    }
}

fn log_errors(mut error_event: EventReader<ProcessError>) {
    for error in error_event.read() {
        eprintln!("{error:?}");
    }
}
