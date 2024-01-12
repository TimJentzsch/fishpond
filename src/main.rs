use std::time::Duration;

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_local_commands::{
    BevyLocalCommandsPlugin, LocalCommand, Process, ProcessError, ProcessOutput,
};

#[derive(Debug, Default, Component)]
struct Game;

#[derive(Debug, Component)]
struct GameRef(Entity);

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Component)]
enum EngineState {
    #[default]
    Startup,
    UciInit,
    Ready,
}

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
        .add_systems(
            Update,
            (
                log_output,
                log_errors,
                handle_engine_startup,
                handle_engine_uci_init,
            ),
        )
        .run();
}

fn start_stockfish(mut commands: Commands) {
    let game_id = commands.spawn(Game).id();
    commands.spawn((
        EngineState::default(),
        GameRef(game_id),
        LocalCommand::new("stockfish"),
    ));
}

fn handle_engine_startup(mut state_query: Query<(&mut EngineState, &mut Process), Added<Process>>) {
    for (mut state, mut process) in state_query.iter_mut() {
        println!("Initializing UCI...");
        process.println("uci").expect("Failed to send uci command");
        *state = EngineState::UciInit;
    }
}

fn handle_engine_uci_init(
    mut output_event: EventReader<ProcessOutput>,
    mut state_query: Query<&mut EngineState>,
) {
    for output in output_event.read() {
        for line in &output.output {
            if line.trim().starts_with("uciok") {
                if let Ok(mut state) = state_query.get_mut(output.entity) {
                    println!("Engine ready!");
                    *state = EngineState::Ready;
                }
            }
        }
    }
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
