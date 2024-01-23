use std::{io::Write, time::Duration};

use bevy::prelude::*;
use bevy_local_commands::{LocalCommand, Process};
use fishpond_game::Game;
use shakmaty::{uci::Uci, Chess};

use crate::{chess::GameRef, process_log::LogSet};

use self::engine_to_gui::{EngineToGuiPlugin, UciToGui};

mod engine_to_gui;
mod uci;

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
    pub game: Game<Chess>,
}

#[derive(Debug, Event)]
pub struct SearchResult {
    pub game_ref: GameRef,
    pub uci_move: Uci,
}

pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EngineToGuiPlugin)
            .add_event::<StartEngine>()
            .add_event::<EngineInitialized>()
            .add_event::<SearchMove>()
            .add_event::<SearchResult>()
            .add_systems(
                Update,
                (
                    handle_start_engine,
                    handle_engine_startup,
                    handle_move_search,
                    handle_engine_to_gui,
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

fn handle_engine_to_gui(
    mut uci_to_gui_event: EventReader<UciToGui>,
    mut state_query: Query<(Entity, &mut EngineState, &GameRef)>,
    mut engine_initialized_event: EventWriter<EngineInitialized>,
    mut search_result_event: EventWriter<SearchResult>,
) {
    for uci_to_gui in uci_to_gui_event.read() {
        let Ok((engine_id, mut state, game_ref)) = state_query.get_mut(uci_to_gui.entity) else {
            continue;
        };

        match &uci_to_gui.command {
            uci::UciToGuiCmd::UciOk => {
                println!("Engine ready!");
                *state = EngineState::Ready;
                engine_initialized_event.send(EngineInitialized {
                    engine_id,
                    game_ref: *game_ref,
                })
            }
            uci::UciToGuiCmd::BestMove { uci_move } => search_result_event.send(SearchResult {
                game_ref: *game_ref,
                uci_move: uci_move.clone(),
            }),
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
            let search_time = Duration::from_millis(200);

            // Search for a fixed time in the current position
            writeln!(
                &mut process,
                "position {}",
                search_move.game.uci_position_with_moves()
            )
            .unwrap();
            writeln!(&mut process, "go movetime {}", search_time.as_millis()).unwrap();
            process.flush().unwrap();
        }
    }
}
