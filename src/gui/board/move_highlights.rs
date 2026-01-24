use bevy::prelude::*;
use fishpond_backend::game::Game;
use shakmaty::Chess;

use crate::gui::board::position::{set_square_position, SQUARE_PERCENT};

#[derive(Component)]
pub struct SourceSquare;

#[derive(Component)]
pub struct TargetSquare;

const LIGHT_HIGHLIGHT_COLOR: Color = Color::srgb_u8(205, 209, 106);
const DARK_HIGHLIGHT_COLOR: Color = Color::srgb_u8(170, 162, 58);

pub fn spawn_move_highlights(commands: &mut EntityCommands) {
    commands.with_children(|builder| {
        builder.spawn((
            SourceSquare,
            Node {
                height: percent(SQUARE_PERCENT),
                width: percent(SQUARE_PERCENT),
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
                height: percent(SQUARE_PERCENT),
                width: percent(SQUARE_PERCENT),
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

pub fn update_move_highlights(
    game_query: Query<&Game<Chess>>,
    mut source_query: Single<
        (&mut BackgroundColor, &mut Node, &mut Visibility),
        (With<SourceSquare>, Without<TargetSquare>),
    >,
    mut target_query: Single<
        (&mut BackgroundColor, &mut Node, &mut Visibility),
        (With<TargetSquare>, Without<SourceSquare>),
    >,
) {
    let Ok(game) = game_query.single() else {
        return;
    };

    if let Some(last_move) = game.moves().last() {
        if let Some(from) = last_move.from() {
            set_square_position(&mut source_query.1, from);

            if from.is_light() {
                source_query.0 .0 = LIGHT_HIGHLIGHT_COLOR;
            } else {
                source_query.0 .0 = DARK_HIGHLIGHT_COLOR;
            }
            *source_query.2 = Visibility::Visible;
        } else {
            *source_query.2 = Visibility::Hidden;
        }

        let to = last_move.to();
        set_square_position(&mut target_query.1, to);

        if to.is_light() {
            target_query.0 .0 = LIGHT_HIGHLIGHT_COLOR;
        } else {
            target_query.0 .0 = DARK_HIGHLIGHT_COLOR;
        }
        *target_query.2 = Visibility::Visible;
    } else {
        *source_query.2 = Visibility::Hidden;
        *target_query.2 = Visibility::Hidden;
    }
}
