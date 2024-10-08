use bevy::prelude::*;

use super::{
    palette::{get_square_image, SquareImageSize},
    piece::Piece,
};

#[derive(Resource)]
pub struct SquareImageAssets {
    standard: Vec<Handle<Image>>,
    small: Vec<Handle<Image>>,
}

impl SquareImageAssets {
    pub fn new(image_assets: &mut Assets<Image>, level: usize) -> Self {
        Self {
            standard: Piece::iter()
                .map(|piece| {
                    image_assets.add(get_square_image(SquareImageSize::Standard, *piece, level))
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
            SquareImageSize::Standard => self.standard[piece.variant_index()].clone(),
            SquareImageSize::Small => self.small[piece.variant_index()].clone(),
        }
    }
}

#[derive(Resource)]
pub struct ColorMaterialAssets {
    pub red: Handle<ColorMaterial>,
    pub white: Handle<ColorMaterial>,
}
