use bevy::prelude::*;
use bevy_vello::prelude::UiVelloSvg;
use fishpond_backend::game::Game;
use shakmaty::{Chess, Position, Square};

use crate::gui::board::position::SQUARE_PERCENT;

#[derive(Component)]
pub struct PieceContainer;

pub fn spawn_pieces(commands: &mut EntityCommands) {
    commands.with_child((
        PieceContainer,
        Node {
            width: percent(100),
            height: percent(100),
            position_type: PositionType::Absolute,
            ..default()
        },
    ));
}

pub fn update_pieces(
    mut commands: Commands,
    game_query: Query<&Game<Chess>>,
    piece_container_query: Single<Entity, With<PieceContainer>>,
    asset_server: Res<AssetServer>,
) {
    let Ok(game) = game_query.single() else {
        return;
    };

    let container = *piece_container_query;
    let mut container_commands = commands.entity(container);

    // Clear existing pieces
    container_commands.despawn_children();

    // Spawn pieces based on the current game state
    for square in Square::ALL {
        if let Some(piece) = game.current_position().board().piece_at(square) {
            let piece_color = match piece.color {
                shakmaty::Color::White => "w",
                shakmaty::Color::Black => "b",
            };
            let piece_type = match piece.role {
                shakmaty::Role::Pawn => "P",
                shakmaty::Role::Knight => "N",
                shakmaty::Role::Bishop => "B",
                shakmaty::Role::Rook => "R",
                shakmaty::Role::Queen => "Q",
                shakmaty::Role::King => "K",
            };
            let piece_image_path = format!("pieces/cburnett/{piece_color}{piece_type}.svg");

            let mut piece_node = Node {
                width: percent(SQUARE_PERCENT),
                height: percent(SQUARE_PERCENT),
                position_type: PositionType::Absolute,
                ..default()
            };
            crate::gui::board::position::set_square_position(&mut piece_node, square);

            container_commands.with_children(|builder| {
                builder.spawn((
                    piece_node,
                    UiVelloSvg(asset_server.load(piece_image_path)),
                    GlobalZIndex(5),
                ));
            });
        }
    }
}
