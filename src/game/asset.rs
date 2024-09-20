use bevy::prelude::*;

use super::{
    palette::{get_square_image, SquareImageSize},
    piece::PieceShape,
};

#[derive(Resource)]
pub struct SquareImageAssets {
    normal: Vec<Handle<Image>>,
    small: Vec<Handle<Image>>,
}

impl SquareImageAssets {
    pub fn new(image_assets: &mut Assets<Image>, level: usize) -> Self {
        Self {
            normal: PieceShape::iter()
                .map(|shape| {
                    image_assets.add(get_square_image(SquareImageSize::Normal, *shape, level))
                })
                .collect(),
            small: PieceShape::iter()
                .map(|shape| {
                    image_assets.add(get_square_image(SquareImageSize::Small, *shape, level))
                })
                .collect(),
        }
    }

    pub fn get_image(&self, size: SquareImageSize, shape: PieceShape) -> Handle<Image> {
        match size {
            SquareImageSize::Normal => self.normal[shape as usize].clone(),
            SquareImageSize::Small => self.small[shape as usize].clone(),
        }
    }
}
