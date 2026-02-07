use bevy::prelude::*;
use fishpond_backend::game::Game;
use shakmaty::{Chess, Position, Square};

use crate::gui::board::position::{SQUARE_PERCENT, set_square_position};

#[derive(Component)]
pub struct PieceContainer;

#[derive(Component)]
pub struct RenderedPosition(Chess);

#[derive(Component)]
pub struct RenderedPiece {
    pub square: Square,
    pub piece: shakmaty::Piece,
}

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
    mut piece_query: Query<(&mut Node, &mut RenderedPiece)>,
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
            let last_move = game.moves().last();
            if let Some(last_move) = last_move
                && let Ok(compare_position) = visualized_position.0.clone().play(last_move)
                && compare_position == *game.current_position()
            {
                visualized_position.0 = game.current_position().clone();

                // Only the last move has to be applied
                if let shakmaty::Move::Normal {
                    role,
                    from,
                    capture,
                    to,
                    promotion,
                } = *last_move
                    && capture.is_none()
                    && promotion.is_none()
                {
                    // Try to find the matching piece
                    for (mut node, mut rendered_piece) in piece_query.iter_mut() {
                        if rendered_piece.square == from && rendered_piece.piece.role == role {
                            // Move the piece to the new square
                            rendered_piece.square = to;
                            set_square_position(&mut node, to);
                            println!("---- EFFICIENT UPDATE ----");
                            return;
                        }
                    }
                }
            }
            visualized_position.0 = game.current_position().clone();
        }
    } else {
        commands
            .entity(container)
            .insert(RenderedPosition(game.current_position().clone()));
    }
    let mut container_commands = commands.entity(container);

    println!("---- INEFFICIENT UPDATE ----");

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
            set_square_position(&mut piece_node, square);

            container_commands.with_children(|builder| {
                builder.spawn((
                    piece_node,
                    RenderedPiece { square, piece },
                    ImageNode::new(asset_server.load(piece_image_path)),
                ));
            });
        }
    }
}
