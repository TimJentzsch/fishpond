use bevy::prelude::*;
use bevy_local_commands::{ProcessError, ProcessOutput};

#[derive(Debug, PartialEq, Eq, Hash, Clone, SystemSet)]
pub(crate) struct LogSet;

pub struct ProcessLogPlugin;

impl Plugin for ProcessLogPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (log_output, log_errors).in_set(LogSet));
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
