use bevy::prelude::*;

use super::board::Board;

const BOARD_BACKGROUND_LAYER: f32 = 1.0;
const BOARD_LAYER: f32 = 2.0;
const BLOCK_LAYER: f32 = 3.0;
const CURR_PIECE_LAYER: f32 = 4.0;
const GAME_PAUSE_SCREEN_LAYER: f32 = 5.0;

pub struct SpawnParam {
    unit: f32,
}

impl SpawnParam {
    pub fn new() -> Self {
        Self { unit: 38.0 }
    }

    pub fn unit(&self) -> f32 {
        self.unit
    }

    pub fn block_size(&self) -> Vec2 {
        Vec2::new(self.unit, self.unit)
    }

    pub fn visible_block_size(&self) -> Vec2 {
        let padding = self.unit / 10.0;
        Vec2::new(self.unit - padding, self.unit - padding)
    }

    pub fn block_translation_offset(&self) -> Vec3 {
        (self.board_size() / -2.0).extend(BLOCK_LAYER)
    }

    pub fn curr_piece_translation(&self) -> Vec3 {
        (self.board_size() / -2.0).extend(CURR_PIECE_LAYER)
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

    pub fn game_pause_screen_translation(&self) -> Vec3 {
        Vec3::new(0.0, 0.0, GAME_PAUSE_SCREEN_LAYER)
    }

    pub fn lines_translation(&self) -> Vec3 {
        Vec3::new(
            0.0,
            self.board_height() / 2.0 + self.unit * 2.0,
            BOARD_LAYER,
        )
    }

    pub fn score_translation(&self) -> Vec3 {
        Vec3::new(self.board_width(), self.board_height() / 3.0, BOARD_LAYER)
    }

    pub fn level_translation(&self) -> Vec3 {
        Vec3::new(self.board_width(), -self.board_height() / 3.0, BOARD_LAYER)
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

    pub fn next_piece_translation(&self) -> Vec3 {
        Vec3::new(self.board_width(), 0.0, CURR_PIECE_LAYER)
    }

    pub fn das_translation(&self) -> Vec3 {
        Vec3::new(-self.board_width(), self.unit * 5.0, BOARD_LAYER)
    }

    pub fn burned_translation(&self) -> Vec3 {
        Vec3::new(-self.board_width(), self.unit * 2.0, BOARD_LAYER)
    }

    pub fn tetris_count_translation(&self) -> Vec3 {
        Vec3::new(-self.board_width(), self.unit * 1.0, BOARD_LAYER)
    }

    pub fn tetris_rate_translation(&self) -> Vec3 {
        Vec3::new(-self.board_width(), self.unit * 0.0, BOARD_LAYER)
    }

    pub fn drought_translation(&self) -> Vec3 {
        Vec3::new(-self.board_width(), -self.unit * 2.0, BOARD_LAYER)
    }
}

impl Default for SpawnParam {
    fn default() -> Self {
        Self::new()
    }
}
