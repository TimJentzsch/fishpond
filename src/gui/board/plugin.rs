use bevy::prelude::*;

use crate::gui::board::{background::spawn_background, move_highlight::update_move_highlights};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_background)
            .add_systems(Update, update_move_highlights);
    }
}
