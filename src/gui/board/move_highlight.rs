use bevy::prelude::*;

#[derive(Component)]
struct SourceSquare;

#[derive(Component)]
struct TargetSquare;

const LIGHT_HIGHLIGHT_COLOR: Color = Color::srgb_u8(205, 209, 106);
const DARK_HIGHLIGHT_COLOR: Color = Color::srgb_u8(170, 162, 58);

pub fn spawn_move_highlight(commands: &mut EntityCommands) {
    commands.with_children(|builder| {
        builder.spawn((
            SourceSquare,
            Node {
                height: percent(100.0 / 8.0),
                width: percent(100.0 / 8.0),
                position_type: PositionType::Absolute,
                top: percent(0.0),
                left: percent(0.0),
                ..default()
            },
            BackgroundColor(LIGHT_HIGHLIGHT_COLOR),
            Visibility::Hidden,
        ));

        builder.spawn((
            TargetSquare,
            Node {
                height: percent(100.0 / 8.0),
                width: percent(100.0 / 8.0),
                position_type: PositionType::Absolute,
                top: percent(0.0),
                left: percent(0.0),
                ..default()
            },
            BackgroundColor(DARK_HIGHLIGHT_COLOR),
            Visibility::Hidden,
        ));
    });
}
