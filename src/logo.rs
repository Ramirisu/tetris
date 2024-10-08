use bevy::prelude::*;

use crate::game::{
    palette::{get_square_image, SquareImageSize},
    piece::Piece,
};

pub fn load_logo_images(image_assets: &mut Assets<Image>) -> [Handle<Image>; 7] {
    [
        image_assets.add(get_square_image(SquareImageSize::Small, Piece::X, 0)),
        image_assets.add(get_square_image(SquareImageSize::Small, Piece::new_j(), 8)),
        image_assets.add(get_square_image(SquareImageSize::Small, Piece::new_t(), 2)),
        image_assets.add(get_square_image(SquareImageSize::Small, Piece::new_z(), 8)),
        image_assets.add(get_square_image(SquareImageSize::Small, Piece::new_z(), 9)),
        image_assets.add(get_square_image(SquareImageSize::Small, Piece::new_z(), 0)),
        image_assets.add(get_square_image(SquareImageSize::Small, Piece::new_z(), 1)),
    ]
}

#[rustfmt::skip]
pub const TETRIS_BITMAP: &[[u8; 21]; 5] = &[
    [1, 1, 1, 0, 2, 2, 2, 0, 3, 3, 3, 0, 4, 4, 4, 0, 5, 0, 6, 6, 6],
    [0, 1, 0, 0, 2, 0, 0, 0, 0, 3, 0, 0, 4, 0, 4, 0, 5, 0, 6, 0, 0],
    [0, 1, 0, 0, 2, 2, 2, 0, 0, 3, 0, 0, 4, 4, 4, 0, 5, 0, 6, 6, 6],
    [0, 1, 0, 0, 2, 0, 0, 0, 0, 3, 0, 0, 4, 4, 0, 0, 5, 0, 0, 0, 6],
    [0, 1, 0, 0, 2, 2, 2, 0, 0, 3, 0, 0, 4, 0, 4, 0, 5, 0, 6, 6, 6],
];
