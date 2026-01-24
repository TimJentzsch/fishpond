use bevy::prelude::*;
use fishpond_backend::game::Game;
use shakmaty::Chess;

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
            let (file, rank) = from.coords();

            source_query.1.left = percent(file as u8 as f32 * 12.5);
            source_query.1.top = percent((7 - rank as u8) as f32 * 12.5);

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

        let (file, rank) = to.coords();
        target_query.1.left = percent(file as u8 as f32 * 12.5);
        target_query.1.top = percent((7 - rank as u8) as f32 * 12.5);

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
