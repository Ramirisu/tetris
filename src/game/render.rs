use bevy::prelude::*;

use super::board::Board;

const BOARD_BACKGROUND_LAYER: f32 = 1.0;
const BOARD_LAYER: f32 = 2.0;
const SQUARE_LAYER: f32 = 3.0;
const CURR_PIECE_LAYER: f32 = 4.0;
const COVER_LAYER: f32 = 5.0;

pub struct RenderConfig {
    unit: f32,
}

impl RenderConfig {
    pub fn new() -> Self {
        Self { unit: 36.0 }
    }

    pub fn unit(&self) -> f32 {
        self.unit
    }

    fn square_width(&self) -> f32 {
        self.unit
    }

    fn square_height(&self) -> f32 {
        self.unit
    }

    pub fn square_size(&self) -> Vec2 {
        Vec2::new(self.square_width(), self.square_height())
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

    pub fn board_cover_size(&self) -> Vec2 {
        Vec2::new(self.unit * 8.0, self.unit * 3.0)
    }

    pub fn board_cover_translation(&self) -> Vec3 {
        Vec3::new(0.0, 0.0, COVER_LAYER)
    }

    pub fn board_square_translation(&self, x: i32, y: i32) -> Vec3 {
        (Vec2::new(
            (x as f32 + 0.5) * self.square_width(),
            (y as f32 + 0.5) * self.square_height(),
        ) + (self.board_size() / -2.0))
            .extend(SQUARE_LAYER)
    }

    pub fn curr_piece_translation(&self, x: i32, y: i32) -> Vec3 {
        (Vec2::new(
            (x as f32 + 0.5) * self.square_width(),
            (y as f32 + 0.5) * self.square_height(),
        ) + (self.board_size() / -2.0))
            .extend(CURR_PIECE_LAYER)
    }

    pub fn lines_translation(&self) -> Vec3 {
        Vec3::new(-self.board_width(), self.unit * 9.0, BOARD_LAYER)
    }

    pub fn score_translation(&self) -> Vec3 {
        Vec3::new(self.board_width(), self.unit * 9.0, BOARD_LAYER)
    }

    pub fn level_translation(&self) -> Vec3 {
        Vec3::new(self.board_width(), -self.unit * 6.0, BOARD_LAYER)
    }

    pub fn next_piece_translation(&self, x: i32, y: i32) -> Vec3 {
        (Vec2::new(
            (x as f32) * self.square_width(),
            (y as f32) * self.square_height(),
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

    pub fn statistics_translation(&self) -> Vec3 {
        Vec3::new(-self.board_width(), self.unit * 3.0, BOARD_LAYER)
    }

    pub fn das_translation(&self) -> Vec3 {
        Vec3::new(self.board_width(), -self.unit * 8.0, BOARD_LAYER)
    }

    pub fn game_stopwatch_translation(&self) -> Vec3 {
        Vec3::new(self.board_width(), -self.unit * 10.0, BOARD_LAYER)
    }

    pub fn piece_count_square_size(&self) -> Vec2 {
        Vec2::splat(self.unit / 2.0)
    }

    pub fn piece_count_translation(&self, index: usize, x: i32, y: i32) -> Vec3 {
        (Vec2::new(x as f32 + 0.5, y as f32) * self.piece_count_square_size()
            + Vec2::new(
                -self.board_width() - self.unit,
                -self.unit * 2.0 - self.unit * 1.5 * index as f32,
            ))
        .extend(BOARD_LAYER)
    }

    pub fn piece_count_counter_translation(&self, index: usize) -> Vec3 {
        Vec3::new(
            -self.board_width() + self.unit * 2.0,
            -self.unit * 2.0 - self.unit * 1.5 * index as f32,
            BOARD_LAYER,
        )
    }
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self::new()
    }
}
