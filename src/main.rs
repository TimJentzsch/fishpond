use bevy::prelude::*;
use bevy_local_commands::BevyLocalCommandsPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BevyLocalCommandsPlugin))
        .run();
}
