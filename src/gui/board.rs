use bevy::prelude::*;

pub fn spawn_board(mut commands: Commands) {
    commands
        .spawn(Node {
            display: Display::Grid,
            height: percent(100),
            // Ensure a square board
            aspect_ratio: Some(1.0),
            grid_template_rows: RepeatedGridTrack::flex(8, 1.0),
            grid_template_columns: RepeatedGridTrack::flex(8, 1.0),
            ..default()
        })
        .with_children(|builder| {
            for row in 0..8 {
                for col in 0..8 {
                    let is_light = (row + col) % 2 == 0;
                    builder.spawn((
                        Node::default(),
                        BackgroundColor(if is_light {
                            Color::srgb(0.93, 0.87, 0.7)
                        } else {
                            Color::srgb(0.2, 0.3, 0.4)
                        }),
                    ));
                }
            }
        });
}
