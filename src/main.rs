use std::time::Duration;

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_local_commands::{BevyLocalCommandsPlugin, LocalCommand, Process, ProcessOutput};

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                // Limit to 30 FPS
                1.0 / 30.0,
            ))),
            BevyLocalCommandsPlugin,
        ))
        .add_systems(Startup, start_stockfish)
        .add_systems(Update, (log_output, uci).chain())
        .run();
}

fn start_stockfish(mut commands: Commands) {
    commands.spawn(LocalCommand::new("stockfish"));
}

fn log_output(mut output_event: EventReader<ProcessOutput>) {
    for output in output_event.read() {
        println!("{}", output.output.join("\n"));
    }
}

fn uci(mut query: Query<&mut Process, Added<Process>>) {
    for mut process in query.iter_mut() {
        process.println("uci").unwrap();
    }
}
