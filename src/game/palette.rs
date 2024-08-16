use bevy::{
    color::palettes::css::{BLUE, GREEN, ORANGE, PURPLE, RED, WHITE, YELLOW},
    prelude::*,
};

use super::piece::PieceShape;

pub fn get_color(shape: PieceShape) -> Color {
    match shape {
        PieceShape::T => PURPLE,
        PieceShape::J => RED,
        PieceShape::Z => BLUE,
        PieceShape::O => YELLOW,
        PieceShape::S => GREEN,
        PieceShape::L => ORANGE,
        PieceShape::I => WHITE,
    }
    .into()
}
