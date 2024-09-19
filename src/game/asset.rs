use bevy::prelude::*;

use super::{palette::SquareImageSize, piece::PieceShape};

#[derive(Resource)]
pub struct SquareImageAssets {
    normal: Vec<Handle<Image>>,
    small: Vec<Handle<Image>>,
}

impl SquareImageAssets {
    pub fn new(normal: Vec<Handle<Image>>, small: Vec<Handle<Image>>) -> Self {
        Self { normal, small }
    }

    pub fn get_image(&self, size: SquareImageSize, shape: PieceShape) -> Handle<Image> {
        match size {
            SquareImageSize::Normal => self.normal[shape as usize].clone(),
            SquareImageSize::Small => self.small[shape as usize].clone(),
        }
    }
}
