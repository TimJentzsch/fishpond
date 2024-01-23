use bevy::prelude::*;
use bevy_local_commands::ProcessOutput;

use super::uci::UciToGuiCmd;

#[derive(Debug, Event)]
pub struct UciToGui {
    pub entity: Entity,
    pub command: UciToGuiCmd,
}

pub struct EngineToGuiPlugin;

impl Plugin for EngineToGuiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<UciToGui>()
            .add_systems(Update, parse_engine_output);
    }
}

/// Read the engine output and parse it to UCI commands.
fn parse_engine_output(
    mut output_event: EventReader<ProcessOutput>,
    mut uci_to_gui_event: EventWriter<UciToGui>,
) {
    for output in output_event.read() {
        for line in output.lines() {
            if let Ok(command) = line.parse::<UciToGuiCmd>() {
                uci_to_gui_event.send(UciToGui {
                    entity: output.entity,
                    command,
                });
            }
        }
    }
}
