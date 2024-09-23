use bevy::prelude::*;

use super::{
    palette::{get_square_image, SquareImageSize},
    piece::Piece,
};

#[derive(Resource)]
pub struct SquareImageAssets {
    normal: Vec<Handle<Image>>,
    small: Vec<Handle<Image>>,
}

impl SquareImageAssets {
    pub fn new(image_assets: &mut Assets<Image>, level: usize) -> Self {
        Self {
            normal: Piece::iter()
                .map(|piece| {
                    image_assets.add(get_square_image(SquareImageSize::Normal, *piece, level))
                })
                .collect(),
            small: Piece::iter()
                .map(|piece| {
                    image_assets.add(get_square_image(SquareImageSize::Small, *piece, level))
                })
                .collect(),
        }
    }

    pub fn get_image(&self, size: SquareImageSize, piece: Piece) -> Handle<Image> {
        match size {
            SquareImageSize::Normal => self.normal[piece.variant_index()].clone(),
            SquareImageSize::Small => self.small[piece.variant_index()].clone(),
        }
    }
}
