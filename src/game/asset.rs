use std::collections::HashMap;

use bevy::prelude::*;

use super::{palette::SquareImageSize, piece::PieceShape};

#[derive(Resource)]
pub struct SquareImageAssets {
    images: HashMap<SquareImageSize, Vec<Handle<Image>>>,
    empty: HashMap<SquareImageSize, Handle<Image>>,
}

impl SquareImageAssets {
    pub fn new(
        images: HashMap<SquareImageSize, Vec<Handle<Image>>>,
        empty: HashMap<SquareImageSize, Handle<Image>>,
    ) -> Self {
        Self { images, empty }
    }

    pub fn get_image(&self, size: SquareImageSize, shape: PieceShape) -> Handle<Image> {
        self.images[&size][shape as usize].clone()
    }

    pub fn get_empty(&self, size: SquareImageSize) -> Handle<Image> {
        self.empty[&size].clone()
    }

    pub fn get_image_or_empty(
        &self,
        size: SquareImageSize,
        shape: Option<PieceShape>,
    ) -> Handle<Image> {
        if let Some(shape) = shape {
            self.get_image(size, shape)
        } else {
            self.get_empty(size)
        }
    }
}
