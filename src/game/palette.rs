use bevy::{
    color::palettes::{
        css::{
            BEIGE, BLACK, BLUE, BLUE_VIOLET, BURLYWOOD, DARK_BLUE, DARK_GRAY, DARK_GREEN,
            DARK_MAGENTA, DARK_ORCHID, DARK_RED, DIM_GRAY, DODGER_BLUE, GREEN, HOT_PINK,
            INDIAN_RED, INDIGO, LIGHT_GRAY, LIGHT_PINK, LIME, LIMEGREEN, MEDIUM_ORCHID,
            MEDIUM_SEA_GREEN, MEDIUM_SLATE_BLUE, NAVY, ORANGE, PURPLE, RED, SADDLE_BROWN, TEAL,
            WHITE,
        },
        tailwind::{AMBER_700, CYAN_950},
    },
    prelude::*,
    render::render_asset::RenderAssetUsages,
};
use image::{DynamicImage, Rgb32FImage};

use super::piece::PieceShape;

pub fn get_square_image(size: SquareImageSize, shape: PieceShape, level: usize) -> Image {
    let palette = get_level_palette(level);
    match shape {
        PieceShape::T => SquareImage::new(size, SquarePattern::X, palette),
        PieceShape::J => SquareImage::new(size, SquarePattern::Z, palette),
        PieceShape::Z => SquareImage::new(size, SquarePattern::Y, palette),
        PieceShape::O => SquareImage::new(size, SquarePattern::X, palette),
        PieceShape::S => SquareImage::new(size, SquarePattern::Z, palette),
        PieceShape::L => SquareImage::new(size, SquarePattern::Y, palette),
        PieceShape::I => SquareImage::new(size, SquarePattern::X, palette),
    }
    .into()
}

pub fn get_empty_square_image(size: SquareImageSize) -> Image {
    SquareImage::new_empty(size).into()
}

fn get_level_palette(level: usize) -> &'static [Srgba; 4] {
    match level % 256 {
        0 | 10 | 20 | 30 | 40 | 50 | 60 | 70 | 80 | 90 | 100 | 110 | 120 | 130 | 192 => {
            &[BLACK, WHITE, DODGER_BLUE, BLUE]
        }
        1 | 11 | 21 | 31 | 41 | 51 | 61 | 71 | 81 | 91 | 101 | 111 | 121 | 131 | 161 | 193
        | 225 => &[BLACK, WHITE, LIME, GREEN],
        2 | 12 | 22 | 32 | 42 | 52 | 62 | 72 | 82 | 92 | 102 | 112 | 122 | 132 | 194 => {
            &[BLACK, WHITE, MEDIUM_ORCHID, DARK_ORCHID]
        }
        3 | 13 | 23 | 33 | 43 | 53 | 63 | 73 | 83 | 93 | 103 | 113 | 123 | 133 | 195 => {
            &[BLACK, WHITE, LIMEGREEN, BLUE]
        }
        4 | 14 | 24 | 34 | 44 | 54 | 64 | 74 | 84 | 94 | 104 | 114 | 124 | 134 | 196 => {
            &[BLACK, WHITE, MEDIUM_SEA_GREEN, DARK_MAGENTA]
        }
        5 | 15 | 25 | 35 | 45 | 55 | 65 | 75 | 85 | 95 | 105 | 115 | 125 | 135 | 197 => {
            &[BLACK, WHITE, MEDIUM_SLATE_BLUE, MEDIUM_SEA_GREEN]
        }
        6 | 16 | 26 | 36 | 46 | 56 | 66 | 76 | 86 | 96 | 106 | 116 | 126 | 136 | 198 => {
            &[BLACK, WHITE, DIM_GRAY, RED]
        }
        7 | 17 | 27 | 37 | 47 | 57 | 67 | 77 | 87 | 97 | 107 | 117 | 127 | 137 | 199 => {
            &[BLACK, WHITE, DARK_RED, BLUE_VIOLET]
        }
        8 | 18 | 28 | 38 | 48 | 58 | 68 | 78 | 88 | 98 | 108 | 118 | 128 | 200 => {
            &[BLACK, WHITE, RED, BLUE]
        }
        9 | 19 | 29 | 39 | 49 | 59 | 69 | 79 | 89 | 99 | 109 | 119 | 129 | 201 => {
            &[BLACK, WHITE, ORANGE, RED]
        }
        138 | 202 => &[BLACK, INDIAN_RED, LIME, HOT_PINK],
        139 | 142 | 203 | 206 => &[BLACK, DARK_GREEN, DARK_ORCHID, WHITE],
        140 | 204 => &[BLACK, LIME, WHITE, DARK_RED],
        141 | 205 => &[BLACK, INDIAN_RED, DARK_GREEN, HOT_PINK],
        143 | 207 => &[BLACK, LIME, WHITE, DARK_RED],
        144 | 208 => &[BLACK, WHITE, HOT_PINK, DARK_GREEN],
        145 | 209 => &[BLACK, WHITE, WHITE, RED],
        146 | 162 | 210 | 226 => &[BLACK, BLACK, DARK_GREEN, DARK_BLUE],
        147 | 211 => &[BLACK, WHITE, HOT_PINK, MEDIUM_ORCHID],
        148 | 212 => &[BLACK, DIM_GRAY, DARK_GRAY, BLACK],
        149 | 213 => &[BLACK, MEDIUM_ORCHID, HOT_PINK, BEIGE],
        150 | 214 => &[BLACK, HOT_PINK, WHITE, MEDIUM_SEA_GREEN],
        151 | 215 => &[BLACK, DARK_RED, INDIAN_RED, CYAN_950],
        152 | 216 => &[BLACK, AMBER_700, HOT_PINK, HOT_PINK],
        153 | 217 => &[BLACK, BEIGE, DARK_GRAY, BURLYWOOD],
        154 | 218 => &[BLACK, MEDIUM_ORCHID, DARK_GREEN, TEAL],
        155 | 219 => &[BLACK, WHITE, LIME, DIM_GRAY],
        156 | 220 => &[BLACK, MEDIUM_ORCHID, DARK_RED, DARK_RED],
        157 | 221 => &[BLACK, DARK_BLUE, LIME, DARK_BLUE],
        158 | 222 => &[BLACK, SADDLE_BROWN, LIME, DARK_RED],
        159 | 223 => &[BLACK, DIM_GRAY, INDIAN_RED, BLACK],
        160 | 224 => &[BLACK, RED, GREEN, DARK_RED],
        163 | 227 => &[BLACK, DARK_RED, HOT_PINK, INDIAN_RED],
        164 | 228 => &[BLACK, BLACK, CYAN_950, INDIAN_RED],
        165 | 229 => &[BLACK, WHITE, MEDIUM_SEA_GREEN, AMBER_700],
        166 | 230 => &[BLACK, BLACK, LIME, DIM_GRAY],
        167 | 231 => &[BLACK, BLACK, WHITE, HOT_PINK],
        168 | 232 => &[BLACK, DARK_GREEN, DARK_RED, DARK_GRAY],
        169 | 233 => &[BLACK, INDIAN_RED, BLUE_VIOLET, INDIAN_RED],
        170 | 234 => &[BLACK, LIGHT_PINK, DIM_GRAY, BLACK],
        171 | 235 => &[BLACK, DARK_GREEN, DARK_GREEN, DARK_GREEN],
        172 | 236 => &[BLACK, DARK_RED, GREEN, LIGHT_PINK],
        173 | 237 => &[BLACK, GREEN, WHITE, WHITE],
        174 | 238 => &[BLACK, GREEN, WHITE, INDIAN_RED],
        175 | 239 => &[BLACK, HOT_PINK, INDIAN_RED, AMBER_700],
        176 | 240 => &[BLACK, DARK_GREEN, LIME, DARK_RED],
        177 | 241 => &[BLACK, DARK_RED, WHITE, SADDLE_BROWN],
        178 | 186 | 242 | 250 => &[BLACK, LIGHT_GRAY, BLACK, GREEN],
        179 | 243 => &[BLACK, GREEN, DARK_GRAY, TEAL],
        180 | 244 => &[BLACK, INDIAN_RED, WHITE, NAVY],
        181 | 245 => &[BLACK, DARK_RED, MEDIUM_SEA_GREEN, HOT_PINK],
        182 | 246 => &[BLACK, LIME, DARK_RED, AMBER_700],
        183 | 247 => &[BLACK, GREEN, DARK_GREEN, DARK_RED],
        184 | 248 => &[BLACK, DARK_RED, BEIGE, LIME],
        185 | 249 => &[BLACK, CYAN_950, LIMEGREEN, GREEN],
        187 | 251 => &[BLACK, GREEN, WHITE, DIM_GRAY],
        188 | 252 => &[BLACK, DIM_GRAY, DIM_GRAY, DARK_BLUE],
        189 | 253 => &[BLACK, DARK_BLUE, DARK_BLUE, NAVY],
        190 | 254 => &[BLACK, INDIGO, PURPLE, PURPLE],
        191 | 255 => &[BLACK, DARK_RED, DARK_RED, DARK_RED],
        256.. => &[BLACK, BLACK, BLACK, BLACK],
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum SquareImageSize {
    Normal,
    Small,
}

impl SquareImageSize {
    pub fn iter() -> std::slice::Iter<'static, SquareImageSize> {
        const SIZES: [SquareImageSize; 2] = [SquareImageSize::Normal, SquareImageSize::Small];
        SIZES.iter()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum SquarePattern {
    X,
    Y,
    Z,
}

struct SquareImage {
    width: usize,
    height: usize,
    buffer: Vec<Srgba>,
}

impl SquareImage {
    pub fn new_empty(size: SquareImageSize) -> Self {
        Self::new(size, SquarePattern::X, &[BLACK, BLACK, BLACK, BLACK])
    }

    pub fn new(size: SquareImageSize, pattern: SquarePattern, colors: &[Srgba; 4]) -> Self {
        match size {
            SquareImageSize::Normal => {
                let pattern = match pattern {
                    SquarePattern::X => Self::SQUARE_PATTERN_X,
                    SquarePattern::Y => Self::SQUARE_PATTERN_Y,
                    SquarePattern::Z => Self::SQUARE_PATTERN_Z,
                };

                Self {
                    width: pattern[0].len(),
                    height: pattern.len(),
                    buffer: pattern
                        .iter()
                        .flat_map(|rows| rows.iter().map(|pattern| colors[*pattern as usize]))
                        .collect(),
                }
            }
            SquareImageSize::Small => {
                let pattern = match pattern {
                    SquarePattern::X => Self::SQUARE_PATTERN_SMALL_X,
                    SquarePattern::Y => Self::SQUARE_PATTERN_SMALL_Y,
                    SquarePattern::Z => Self::SQUARE_PATTERN_SMALL_Z,
                };

                Self {
                    width: pattern[0].len(),
                    height: pattern.len(),
                    buffer: pattern
                        .iter()
                        .flat_map(|rows| rows.iter().map(|pattern| colors[*pattern as usize]))
                        .collect(),
                }
            }
        }
    }

    const SQUARE_PATTERN_X: &'static [[u8; 18]; 18] = &[
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0],
        [0, 1, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0],
        [0, 3, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 3, 0],
        [0, 3, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 3, 0],
        [0, 3, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 3, 0],
        [0, 3, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 3, 0],
        [0, 3, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 3, 0],
        [0, 3, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 3, 0],
        [0, 3, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 3, 0],
        [0, 3, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 3, 0],
        [0, 3, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 3, 0],
        [0, 3, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 3, 0],
        [0, 3, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 3, 0],
        [0, 3, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 3, 0],
        [0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0],
        [0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];

    const SQUARE_PATTERN_SMALL_X: &'static [[u8; 12]; 12] = &[
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 1, 3, 3, 3, 3, 3, 3, 3, 3, 0],
        [0, 1, 1, 3, 3, 3, 3, 3, 3, 3, 3, 0],
        [0, 3, 3, 1, 1, 1, 1, 1, 1, 3, 3, 0],
        [0, 3, 3, 1, 1, 1, 1, 1, 1, 3, 3, 0],
        [0, 3, 3, 1, 1, 1, 1, 1, 1, 3, 3, 0],
        [0, 3, 3, 1, 1, 1, 1, 1, 1, 3, 3, 0],
        [0, 3, 3, 1, 1, 1, 1, 1, 1, 3, 3, 0],
        [0, 3, 3, 1, 1, 1, 1, 1, 1, 3, 3, 0],
        [0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0],
        [0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];

    const SQUARE_PATTERN_Y: &'static [[u8; 18]; 18] = &[
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0],
        [0, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0],
        [0, 2, 2, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0],
        [0, 2, 2, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0],
        [0, 2, 2, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0],
        [0, 2, 2, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0],
        [0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0],
        [0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0],
        [0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0],
        [0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0],
        [0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0],
        [0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0],
        [0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0],
        [0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0],
        [0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0],
        [0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];

    const SQUARE_PATTERN_SMALL_Y: &'static [[u8; 12]; 12] = &[
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 0],
        [0, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 0],
        [0, 2, 2, 1, 1, 1, 1, 2, 2, 2, 2, 0],
        [0, 2, 2, 1, 1, 1, 1, 2, 2, 2, 2, 0],
        [0, 2, 2, 1, 1, 2, 2, 2, 2, 2, 2, 0],
        [0, 2, 2, 1, 1, 2, 2, 2, 2, 2, 2, 0],
        [0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0],
        [0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0],
        [0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0],
        [0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];

    const SQUARE_PATTERN_Z: &'static [[u8; 18]; 18] = &[
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0],
        [0, 1, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0],
        [0, 3, 3, 1, 1, 1, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0],
        [0, 3, 3, 1, 1, 1, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0],
        [0, 3, 3, 1, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0],
        [0, 3, 3, 1, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0],
        [0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0],
        [0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0],
        [0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0],
        [0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0],
        [0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0],
        [0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0],
        [0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0],
        [0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0],
        [0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0],
        [0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];

    const SQUARE_PATTERN_SMALL_Z: &'static [[u8; 12]; 12] = &[
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 1, 3, 3, 3, 3, 3, 3, 3, 3, 0],
        [0, 1, 1, 3, 3, 3, 3, 3, 3, 3, 3, 0],
        [0, 3, 3, 1, 1, 1, 1, 3, 3, 3, 3, 0],
        [0, 3, 3, 1, 1, 1, 1, 3, 3, 3, 3, 0],
        [0, 3, 3, 1, 1, 3, 3, 3, 3, 3, 3, 0],
        [0, 3, 3, 1, 1, 3, 3, 3, 3, 3, 3, 0],
        [0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0],
        [0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0],
        [0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0],
        [0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];
}

impl Into<Image> for SquareImage {
    fn into(self) -> Image {
        Image::from_dynamic(
            DynamicImage::ImageRgb32F(
                Rgb32FImage::from_vec(
                    self.width as u32,
                    self.height as u32,
                    self.buffer
                        .iter()
                        .flat_map(|color| color.to_f32_array_no_alpha())
                        .collect(),
                )
                .unwrap(),
            ),
            true,
            RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
        )
    }
}