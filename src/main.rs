use std::time::Duration;

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use fishpond_backend::FishpondBackendPlugin;

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                // Limit to 30 FPS
                1.0 / 30.0,
            ))),
            FishpondBackendPlugin,
        ))
        .run();
}
