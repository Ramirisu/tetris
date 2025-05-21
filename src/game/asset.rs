use bevy::{
    color::palettes::css::{BLACK, RED, WHITE, YELLOW},
    prelude::*,
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use super::{
    level::Level,
    palette::{SquareImagePattern, SquareImageSize, get_square_image_by_level},
    piece::Piece,
};

#[derive(Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum SquareImageDisplayLevel {
    Info,
    Warn,
    Error,
}

impl SquareImageDisplayLevel {
    pub fn color(&self) -> Srgba {
        match *self {
            Self::Info => WHITE,
            Self::Warn => YELLOW,
            Self::Error => RED,
        }
    }
}

#[derive(Resource)]
pub struct SquareImageAssets {
    standard: Vec<Handle<Image>>, // [Handle<Image>; Piece::variant_size()]
    small: Vec<Handle<Image>>,    // [Handle<Image>; Piece::variant_size()]
    level: Vec<Handle<Image>>,    // [Handle<Image>; SquareImageDisplayLevel::variant_size()]
    burned: Handle<Image>,
}

impl SquareImageAssets {
    pub fn new(image_assets: &mut Assets<Image>, level: Level) -> Self {
        Self {
            standard: Piece::iter()
                .map(|piece| {
                    image_assets.add(get_square_image_by_level(
                        SquareImageSize::Standard,
                        *piece,
                        level,
                    ))
                })
                .collect(),
            small: Piece::iter()
                .map(|piece| {
                    image_assets.add(get_square_image_by_level(
                        SquareImageSize::Small,
                        *piece,
                        level,
                    ))
                })
                .collect(),
            level: SquareImageDisplayLevel::iter()
                .map(|level| {
                    image_assets.add(SquareImagePattern::X.to_image(
                        SquareImageSize::Small,
                        &[BLACK, level.color(), level.color(), level.color()],
                    ))
                })
                .collect(),
            burned: image_assets.add(
                SquareImagePattern::X.to_image(SquareImageSize::Small, &[BLACK, BLACK, BLACK, RED]),
            ),
        }
    }

    pub fn get_image(&self, size: SquareImageSize, piece: Piece) -> Handle<Image> {
        match size {
            SquareImageSize::Standard => self.standard[piece.variant_index()].clone(),
            SquareImageSize::Small => self.small[piece.variant_index()].clone(),
        }
    }

    pub fn get_display_level_image(&self, level: SquareImageDisplayLevel) -> Handle<Image> {
        self.level[level as usize].clone()
    }

    pub fn get_burned_image(&self) -> Handle<Image> {
        self.burned.clone()
    }
}
