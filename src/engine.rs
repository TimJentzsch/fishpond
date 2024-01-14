use bevy::prelude::*;
use bevy_local_commands::{LocalCommand, Process, ProcessOutput};

use crate::{game::GameRef, process_log::LogSet};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Component)]
enum EngineState {
    #[default]
    Startup,
    UciInit,
    Ready,
}

#[derive(Debug, Component, Event)]
pub struct StartEngine {
    pub game_ref: GameRef,
    pub path: String,
}

#[derive(Debug, Component, Event)]
pub struct EngineInitialized {
    pub engine_id: Entity,
    pub game_ref: GameRef,
}

pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StartEngine>().add_systems(
            Update,
            (
                handle_start_engine,
                handle_engine_startup,
                handle_engine_uci_init,
            )
                .after(LogSet),
        );
    }
}

fn handle_start_engine(mut start_engine_event: EventReader<StartEngine>, mut commands: Commands) {
    for start_engine in start_engine_event.read() {
        commands.spawn((
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
