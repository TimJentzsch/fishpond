use std::time::Duration;

use crate::game::Game;
use bevy::prelude::*;
use bevy_local_commands::{LocalCommand, Process};
use shakmaty::{uci::UciMove, Chess};

use crate::{chess::GameRef, process_log::LogSet};

use self::{
    engine_to_gui::{EngineToGuiPlugin, UciToGui},
    gui_to_engine::{GuiToEnginePlugin, UciToEngine},
};

mod engine_to_gui;
mod gui_to_engine;
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

#[derive(Debug, Component, Default)]
struct EngineId {
    name: Option<String>,
    author: Option<String>,
}

#[derive(Debug, Message)]
pub struct StartEngine {
    pub game_ref: GameRef,
    pub path: String,
}

#[derive(Debug, Message)]
pub struct EngineInitialized {
    #[allow(dead_code)]
    pub engine_id: Entity,
    pub game_ref: GameRef,
}

#[derive(Debug, Message)]
pub struct SearchMove {
    pub game_ref: GameRef,
    pub game: Game<Chess>,
}

#[derive(Debug, Message)]
pub struct SearchResult {
    pub game_ref: GameRef,
    pub uci_move: UciMove,
}

pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EngineToGuiPlugin, GuiToEnginePlugin))
            .add_message::<StartEngine>()
            .add_message::<EngineInitialized>()
            .add_message::<SearchMove>()
            .add_message::<SearchResult>()
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

fn handle_start_engine(mut start_engine_event: MessageReader<StartEngine>, mut commands: Commands) {
    for start_engine in start_engine_event.read() {
        commands.spawn((
            Engine,
            EngineState::default(),
            EngineId::default(),
            start_engine.game_ref,
            LocalCommand::new(start_engine.path.clone()),
        ));
    }
}

fn handle_engine_startup(
    mut state_query: Query<(Entity, &mut EngineState), Added<Process>>,
    mut uci_to_engine_event: MessageWriter<UciToEngine>,
) {
    for (entity, mut state) in state_query.iter_mut() {
        println!("Initializing UCI...");
        *state = EngineState::UciInit;
        uci_to_engine_event.write(UciToEngine {
            entity,
            command: uci::UciToEngineCmd::Uci,
        });
    }
}

fn handle_engine_to_gui(
    mut uci_to_gui_event: MessageReader<UciToGui>,
    mut state_query: Query<(Entity, &mut EngineState, &mut EngineId, &GameRef)>,
    mut engine_initialized_event: MessageWriter<EngineInitialized>,
    mut search_result_event: MessageWriter<SearchResult>,
) {
    for uci_to_gui in uci_to_gui_event.read() {
        let Ok((engine_id, mut state, mut id, game_ref)) = state_query.get_mut(uci_to_gui.entity)
        else {
            continue;
        };

        match &uci_to_gui.command {
            uci::UciToGuiCmd::UciOk => {
                println!("Engine ready!");
                *state = EngineState::Ready;
                engine_initialized_event.write(EngineInitialized {
                    engine_id,
                    game_ref: *game_ref,
                });
            }
            uci::UciToGuiCmd::Id { name, author } => {
                if name.is_some() {
                    id.name = name.clone();
                }
                if author.is_some() {
                    id.author = author.clone();
                }
                println!("Updated engine ID to {id:?}");
            }
            uci::UciToGuiCmd::BestMove { uci_move } => {
                search_result_event.write(SearchResult {
                    game_ref: *game_ref,
                    uci_move: *uci_move,
                });
            }
        }
    }
}

fn handle_move_search(
    mut search_move_event: MessageReader<SearchMove>,
    mut engine_query: Query<(Entity, &GameRef), With<Engine>>,
    mut uci_to_engine_event: MessageWriter<UciToEngine>,
) {
    for search_move in search_move_event.read() {
        if let Some((entity, _)) = engine_query
            .iter_mut()
            .find(|(_, game_ref)| search_move.game_ref == **game_ref)
        {
            let move_time = Duration::from_millis(200);

            uci_to_engine_event.write(UciToEngine {
                entity,
                command: uci::UciToEngineCmd::Position {
                    game: Box::new(search_move.game.clone()),
                },
            });
            uci_to_engine_event.write(UciToEngine {
                entity,
                command: uci::UciToEngineCmd::Go { move_time },
            });
        }
    }
}
