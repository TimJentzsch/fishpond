use bevy::prelude::*;
use fishpond_backend::FishpondBackendPlugin;

use crate::gui::GuiPlugin;

mod gui;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FishpondBackendPlugin, GuiPlugin))
        .run();
}
