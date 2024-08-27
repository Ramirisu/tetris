use bevy::{
    color::palettes::css::{
        BLACK, BLUE, BLUE_VIOLET, DARK_MAGENTA, DARK_RED, DIM_GRAY, DODGER_BLUE, GREEN, LIME,
        LIMEGREEN, MEDIUM_SEA_GREEN, MEDIUM_SLATE_BLUE, ORANGE, ORCHID, PURPLE, RED, WHITE,
    },
    prelude::*,
    render::render_asset::RenderAssetUsages,
};
use image::{DynamicImage, Rgb32FImage};

use super::piece::PieceShape;

pub fn get_square_image(shape: PieceShape, level: usize) -> Image {
    let palette = get_level_palette(level);
    match shape {
        PieceShape::T => get_square_image_from_pattern(SquarePattern::X, palette),
        PieceShape::J => get_square_image_from_pattern(SquarePattern::Z, palette),
        PieceShape::Z => get_square_image_from_pattern(SquarePattern::Y, palette),
        PieceShape::O => get_square_image_from_pattern(SquarePattern::X, palette),
        PieceShape::S => get_square_image_from_pattern(SquarePattern::Z, palette),
        PieceShape::L => get_square_image_from_pattern(SquarePattern::Y, palette),
        PieceShape::I => get_square_image_from_pattern(SquarePattern::X, palette),
    }
}

pub fn get_default_square_image() -> Image {
    get_square_image_from_pattern(SquarePattern::X, &[BLACK, BLACK, BLACK, BLACK])
}

fn get_level_palette(level: usize) -> &'static [Srgba; 4] {
    const PALETTES: [[Srgba; 4]; 10] = [
        [BLACK, WHITE, DODGER_BLUE, BLUE],
        [BLACK, WHITE, LIME, GREEN],
        [BLACK, WHITE, ORCHID, PURPLE],
        [BLACK, WHITE, LIMEGREEN, BLUE],
        [BLACK, WHITE, MEDIUM_SEA_GREEN, DARK_MAGENTA],
        [BLACK, WHITE, MEDIUM_SLATE_BLUE, MEDIUM_SEA_GREEN],
        [BLACK, WHITE, DIM_GRAY, RED],
        [BLACK, WHITE, DARK_RED, BLUE_VIOLET],
        [BLACK, WHITE, RED, BLUE],
        [BLACK, WHITE, ORANGE, RED],
    ];
    &PALETTES[level % 10]
}

fn get_square_image_from_pattern(pattern: SquarePattern, colors: &[Srgba; 4]) -> Image {
    let image: DynamicImage = SquareImage::new(pattern, colors).into();
    Image::from_dynamic(
        image,
        true,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
}

#[derive(Clone, Copy)]
enum SquarePattern {
    X,
    Y,
    Z,
}

struct SquareImage {
    buffer: [[Srgba; 8]; 8],
}

impl SquareImage {
    pub fn new(pattern: SquarePattern, colors: &[Srgba; 4]) -> Self {
        let pattern = match pattern {
            SquarePattern::X => &Self::SQUARE_PATTERN_X,
            SquarePattern::Y => &Self::SQUARE_PATTERN_Y,
            SquarePattern::Z => &Self::SQUARE_PATTERN_Z,
        };
        let mut buffer = [[Srgba::BLACK; 8]; 8];
        for y in 0..8 {
            for x in 0..8 {
                buffer[y][x] = colors[pattern[y][x] as usize];
            }
        }
        Self { buffer }
    }

    const SQUARE_PATTERN_X: [[u8; 8]; 8] = [
        [1, 3, 3, 3, 3, 3, 3, 3],
        [3, 1, 1, 1, 1, 1, 1, 3],
        [3, 1, 1, 1, 1, 1, 1, 3],
        [3, 1, 1, 1, 1, 1, 1, 3],
        [3, 1, 1, 1, 1, 1, 1, 3],
        [3, 1, 1, 1, 1, 1, 1, 3],
        [3, 1, 1, 1, 1, 1, 1, 3],
        [3, 3, 3, 3, 3, 3, 3, 3],
    ];

    const SQUARE_PATTERN_Y: [[u8; 8]; 8] = [
        [1, 2, 2, 2, 2, 2, 2, 2],
        [2, 1, 1, 2, 2, 2, 2, 2],
        [2, 1, 2, 2, 2, 2, 2, 2],
        [2, 2, 2, 2, 2, 2, 2, 2],
        [2, 2, 2, 2, 2, 2, 2, 2],
        [2, 2, 2, 2, 2, 2, 2, 2],
        [2, 2, 2, 2, 2, 2, 2, 2],
        [2, 2, 2, 2, 2, 2, 2, 2],
    ];

    const SQUARE_PATTERN_Z: [[u8; 8]; 8] = [
        [1, 3, 3, 3, 3, 3, 3, 3],
        [3, 1, 1, 3, 3, 3, 3, 3],
        [3, 1, 3, 3, 3, 3, 3, 3],
        [3, 3, 3, 3, 3, 3, 3, 3],
        [3, 3, 3, 3, 3, 3, 3, 3],
        [3, 3, 3, 3, 3, 3, 3, 3],
        [3, 3, 3, 3, 3, 3, 3, 3],
        [3, 3, 3, 3, 3, 3, 3, 3],
    ];
}

impl Into<DynamicImage> for SquareImage {
    fn into(self) -> DynamicImage {
        DynamicImage::ImageRgb32F(
            Rgb32FImage::from_vec(
                8,
                8,
                self.buffer
                    .iter()
                    .flat_map(|rows| rows.iter().flat_map(|color| color.to_f32_array_no_alpha()))
                    .collect(),
            )
            .unwrap(),
        )
    }
}
