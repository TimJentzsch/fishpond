use bevy::prelude::*;

use crate::gui::board::spawn_board;

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup, spawn_board));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
