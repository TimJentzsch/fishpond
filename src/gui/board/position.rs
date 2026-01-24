use bevy::prelude::*;
use shakmaty::Square;

pub const SQUARE_PERCENT: f32 = 100.0 / 8.0;

pub fn set_square_position(node: &mut Node, square: Square) {
    node.left = percent(square.file() as u8 as f32 * SQUARE_PERCENT);
    node.top = percent((7 - square.rank() as u8) as f32 * SQUARE_PERCENT);
}
