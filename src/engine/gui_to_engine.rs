use std::io::Write;

use bevy::prelude::*;
use bevy_local_commands::Process;

use super::{uci::UciToEngineCmd, Engine};

#[derive(Debug, Event)]
pub struct UciToEngine {
    pub entity: Entity,
    pub command: UciToEngineCmd,
}

pub struct GuiToEnginePlugin;

impl Plugin for GuiToEnginePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<UciToEngine>()
            .add_systems(Update, write_gui_commands);
    }
}

/// Write commands from the GUI in the engine input.
fn write_gui_commands(
    mut uci_to_gui_event: EventReader<UciToEngine>,
    mut process_query: Query<&mut Process, With<Engine>>,
) {
    for message in uci_to_gui_event.read() {
        if let Ok(mut process) = process_query.get_mut(message.entity) {
            writeln!(&mut process, "{}", message.command).expect("Failed to write input to engine");
            process.flush().expect("Failed to flush process input");
        }
    }
}
