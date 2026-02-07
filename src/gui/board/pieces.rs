use bevy::prelude::*;
use fishpond_backend::game::Game;
use shakmaty::{Chess, Position, Square};

use crate::gui::board::position::SQUARE_PERCENT;

#[derive(Component)]
pub struct PieceContainer;

#[derive(Component)]
pub struct RenderedPosition(Chess);

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
    mut piece_container_query: Query<(Entity, Option<&mut RenderedPosition>), With<PieceContainer>>,
    asset_server: Res<AssetServer>,
) {
    let Ok(game) = game_query.single() else {
        return;
    };
    let Ok((container, mut visualized_position)) = piece_container_query.single_mut() else {
        return;
    };

    if let Some(visualized_position) = &mut visualized_position {
        if visualized_position.0 == *game.current_position() {
            // No change in position, no need to update pieces
            return;
        } else {
            visualized_position.0 = game.current_position().clone();
        }
    } else {
        commands
            .entity(container)
            .insert(RenderedPosition(game.current_position().clone()));
    }

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
            let piece_image_path = format!("pieces/cburnett/{piece_color}{piece_type}.png");

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
                    ImageNode::new(asset_server.load(piece_image_path)),
                ));
            });
        }
    }
}
