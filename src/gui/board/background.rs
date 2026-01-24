use bevy::prelude::*;

use crate::gui::board::move_highlight::spawn_move_highlight;

const LIGHT_SQUARE_COLOR: Color = Color::srgb_u8(240, 217, 181);
const DARK_SQUARE_COLOR: Color = Color::srgb_u8(181, 136, 99);

pub fn spawn_background(mut commands: Commands) {
    let mut board_commands = commands.spawn(Node {
        display: Display::Grid,
        height: percent(100),
        // Ensure a square board
        aspect_ratio: Some(1.0),
        grid_template_rows: RepeatedGridTrack::flex(8, 1.0),
        grid_template_columns: RepeatedGridTrack::flex(8, 1.0),
        ..default()
    });
    board_commands.with_children(|builder| {
        for row in 0..8 {
            for col in 0..8 {
                let is_light = (row + col) % 2 == 0;
                builder.spawn((
                    Node::default(),
                    BackgroundColor(if is_light {
                        LIGHT_SQUARE_COLOR
                    } else {
                        DARK_SQUARE_COLOR
                    }),
                ));
            }
        }
    });
    spawn_move_highlight(&mut board_commands);
}
