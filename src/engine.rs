use std::io::Write;

use bevy::prelude::*;
use bevy_local_commands::{LocalCommand, Process, ProcessOutput};

use crate::{
    game::{GameBoard, GameRef},
    process_log::LogSet,
};

#[derive(Debug, Component)]
struct Engine;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Component)]
enum EngineState {
    #[default]
    Startup,
    UciInit,
    Ready,
}

#[derive(Debug, Event)]
pub struct StartEngine {
    pub game_ref: GameRef,
    pub path: String,
}

#[derive(Debug, Event)]
pub struct EngineInitialized {
    pub engine_id: Entity,
    pub game_ref: GameRef,
}

#[derive(Debug, Event)]
pub struct SearchMove {
    pub game_ref: GameRef,
    pub game_board: GameBoard,
}

pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StartEngine>()
            .add_event::<EngineInitialized>()
            .add_event::<SearchMove>()
            .add_systems(
                Update,
                (
                    handle_start_engine,
                    handle_engine_startup,
                    handle_engine_uci_init,
                    handle_move_search,
                )
                    .after(LogSet),
            );
    }
}

fn handle_start_engine(mut start_engine_event: EventReader<StartEngine>, mut commands: Commands) {
    for start_engine in start_engine_event.read() {
        commands.spawn((
            Engine,
            EngineState::default(),
            start_engine.game_ref,
            LocalCommand::new(start_engine.path.clone()),
        ));
    }
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
    mut state_query: Query<(Entity, &mut EngineState, &GameRef)>,
    mut engine_initialized_event: EventWriter<EngineInitialized>,
) {
    for output in output_event.read() {
        for line in output.lines() {
            if line.trim().starts_with("uciok") {
                if let Ok((engine_id, mut state, game_ref)) = state_query.get_mut(output.entity) {
                    println!("Engine ready!");
                    *state = EngineState::Ready;
                    engine_initialized_event.send(EngineInitialized {
                        engine_id,
                        game_ref: *game_ref,
                    })
                }
            }
        }
    }
}

fn handle_move_search(
    mut search_move_event: EventReader<SearchMove>,
    mut engine_query: Query<(&mut Process, &GameRef), With<Engine>>,
) {
    for search_move in search_move_event.read() {
        if let Some((mut process, _)) = engine_query
            .iter_mut()
            .find(|(_, game_ref)| search_move.game_ref == **game_ref)
        {
            // Search for one second in the current position
            writeln!(&mut process, "position {}", search_move.game_board.fen()).unwrap();
            writeln!(&mut process, "go movetime 1000").unwrap();
            process.flush().unwrap();
        }
    }
}
