use bevy::prelude::*;

use super::board::Board;

const BOARD_BACKGROUND_LAYER: f32 = 1.0;
const BOARD_LAYER: f32 = 2.0;
const BLOCK_LAYER: f32 = 3.0;
const CURR_PIECE_LAYER: f32 = 4.0;
const COVER_LAYER: f32 = 5.0;

pub struct RenderConfig {
    unit: f32,
}

impl RenderConfig {
    pub fn new() -> Self {
        Self { unit: 40.0 }
    }

    pub fn unit(&self) -> f32 {
        self.unit
    }

    fn block_width(&self) -> f32 {
        self.unit
    }

    fn block_height(&self) -> f32 {
        self.unit
    }

    pub fn block_size(&self) -> Vec2 {
        Vec2::new(self.block_width(), self.block_height())
    }

    pub fn visible_block_size(&self) -> Vec2 {
        let padding = self.unit / 10.0;
        Vec2::new(self.unit - padding, self.unit - padding)
    }

    pub fn board_translation(&self) -> Vec3 {
        Vec3::new(0.0, 0.0, BOARD_LAYER)
    }

    fn board_width(&self) -> f32 {
        self.unit * Board::BOARD_COLS as f32
    }

    fn board_height(&self) -> f32 {
        self.unit * Board::BOARD_ROWS as f32
    }

    pub fn board_size(&self) -> Vec2 {
        Vec2::new(self.board_width(), self.board_height())
    }

    pub fn board_background_size(&self) -> Vec2 {
        Vec2::new(
            self.board_width() + self.unit / 10.0,
            self.board_height() + self.unit / 20.0,
        )
    }

    pub fn board_background_translation(&self) -> Vec3 {
        Vec3::new(0.0, -self.unit / 20.0, BOARD_BACKGROUND_LAYER)
    }

    pub fn board_cover_translation(&self) -> Vec3 {
        Vec3::new(0.0, 0.0, COVER_LAYER)
    }

    pub fn board_block_translation(&self, x: i32, y: i32) -> Vec3 {
        (Vec2::new(
            (x as f32 + 0.5) * self.block_width(),
            (y as f32 + 0.5) * self.block_height(),
        ) + (self.board_size() / -2.0))
            .extend(BLOCK_LAYER)
    }

    pub fn curr_piece_translation(&self, x: i32, y: i32) -> Vec3 {
        (Vec2::new(
            (x as f32 + 0.5) * self.block_width(),
            (y as f32 + 0.5) * self.block_height(),
        ) + (self.board_size() / -2.0))
            .extend(CURR_PIECE_LAYER)
    }

    pub fn lines_translation(&self) -> Vec3 {
        Vec3::new(-self.board_width(), self.board_height() / 3.0, BOARD_LAYER)
    }

    pub fn score_translation(&self) -> Vec3 {
        Vec3::new(self.board_width(), self.board_height() / 3.0, BOARD_LAYER)
    }

    pub fn level_translation(&self) -> Vec3 {
        Vec3::new(self.board_width(), -self.board_height() / 3.0, BOARD_LAYER)
    }

    pub fn next_piece_translation(&self, x: i32, y: i32) -> Vec3 {
        (Vec2::new(
            (x as f32) * self.block_width(),
            (y as f32) * self.block_height(),
        ) + Vec2::new(self.board_width(), 0.0))
        .extend(CURR_PIECE_LAYER)
    }

    pub fn next_piece_slot_translation(&self) -> Vec3 {
        Vec3::new(self.board_width(), 0.0, BOARD_LAYER)
    }

    pub fn next_piece_slot_size(&self) -> Vec2 {
        Vec2::new(self.unit * 6.0, self.unit * 6.0)
    }

    pub fn next_piece_slot_background_translation(&self) -> Vec3 {
        Vec3::new(self.board_width(), 0.0, BOARD_BACKGROUND_LAYER)
    }

    pub fn next_piece_slot_background_size(&self) -> Vec2 {
        Vec2::new(self.unit * 6.1, self.unit * 6.1)
    }

    pub fn next_piece_slot_cover_translation(&self) -> Vec3 {
        Vec3::new(self.board_width(), 0.0, COVER_LAYER)
    }

    pub fn tetris_translation(&self) -> Vec3 {
        Vec3::new(-self.board_width(), self.unit * 1.0, BOARD_LAYER)
    }

    pub fn das_translation(&self) -> Vec3 {
        Vec3::new(-self.board_width(), -self.unit * 4.0, BOARD_LAYER)
    }
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self::new()
    }
}
