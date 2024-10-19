use bevy::prelude::*;

use super::board::Board;

const FLASH_Z_INDEX: f32 = 0.0;
const BOARD_BACKGROUND_Z_INDEX: f32 = 1.0;
const BOARD_Z_INDEX: f32 = 2.0;
const SQUARE_Z_INDEX: f32 = 3.0;
const CURR_PIECE_Z_INDEX: f32 = 4.0;
const COVER_Z_INDEX: f32 = 5.0;

#[derive(Clone, Copy, Resource)]
pub struct GameTransform {
    scale: f32,
}

impl GameTransform {
    pub fn new(scale: f32) -> Self {
        Self { scale }
    }

    pub fn fs_medium(&self) -> f32 {
        self.scale * 36.0
    }

    pub fn fs_large(&self) -> f32 {
        self.scale * 48.0
    }

    pub fn fs_xlarge(&self) -> f32 {
        self.scale * 72.0
    }

    fn square_width(&self) -> f32 {
        self.scale * 36.0
    }

    fn square_height(&self) -> f32 {
        self.scale * 36.0
    }

    pub fn square_size(&self) -> Vec2 {
        Vec2::new(self.square_width(), self.square_height())
    }

    fn border_width(&self) -> f32 {
        self.scale * 2.0
    }

    fn border_size(&self) -> Vec2 {
        Vec2::splat(self.border_width())
    }

    // center

    pub fn flash_size(&self) -> Vec2 {
        Vec2::new(self.board_width() * 4.0, self.board_height() * 2.0)
    }

    pub fn flash_translation(&self) -> Vec3 {
        Vec3::new(0.0, 0.0, FLASH_Z_INDEX)
    }

    pub fn board_translation(&self) -> Vec3 {
        Vec3::new(0.0, 0.0, BOARD_Z_INDEX)
    }

    fn board_width(&self) -> f32 {
        self.square_width() * Board::BOARD_COLS as f32
    }

    fn board_height(&self) -> f32 {
        self.square_height() * Board::BOARD_ROWS as f32
    }

    pub fn board_size(&self) -> Vec2 {
        Vec2::new(self.board_width(), self.board_height())
    }

    pub fn board_background_size(&self) -> Vec2 {
        Vec2::new(
            self.board_width() + self.border_width() * 2.0,
            self.board_height() + self.border_width(),
        )
    }

    pub fn board_background_translation(&self) -> Vec3 {
        Vec3::new(0.0, -self.border_width(), BOARD_BACKGROUND_Z_INDEX)
    }

    pub fn board_cover_size(&self) -> Vec2 {
        Vec2::new(self.square_width() * 8.0, self.square_height() * 3.0)
    }

    pub fn board_cover_translation(&self) -> Vec3 {
        Vec3::new(0.0, 0.0, COVER_Z_INDEX)
    }

    pub fn board_square_translation(&self, x: i32, y: i32) -> Vec3 {
        (Vec2::new(
            (x as f32 + 0.5) * self.square_width(),
            (y as f32 + 0.5) * self.square_height(),
        ) + (self.board_size() / -2.0))
            .extend(SQUARE_Z_INDEX)
    }

    pub fn curr_piece_translation(&self, x: i32, y: i32) -> Vec3 {
        (Vec2::new(
            (x as f32 + 0.5) * self.square_width(),
            (y as f32 + 0.5) * self.square_height(),
        ) + (self.board_size() / -2.0))
            .extend(CURR_PIECE_Z_INDEX)
    }

    pub fn das_translation(&self) -> Vec3 {
        Vec3::new(
            0.0,
            -self.board_height() / 2.0 - self.square_height(),
            BOARD_Z_INDEX,
        )
    }

    // left

    pub fn lines_translation(&self) -> Vec3 {
        Vec3::new(
            -self.board_width(),
            self.square_height() * 9.0,
            BOARD_Z_INDEX,
        )
    }

    pub fn statistics_translation(&self) -> Vec3 {
        Vec3::new(
            -self.board_width(),
            self.square_height() * 3.0,
            BOARD_Z_INDEX,
        )
    }

    pub fn piece_count_square_size(&self) -> Vec2 {
        Vec2::new(self.square_width() * 0.5, self.square_height() * 0.5)
    }

    pub fn piece_count_translation(&self, index: usize, x: f32, y: f32) -> Vec3 {
        (Vec2::new(x + 0.5, y) * self.piece_count_square_size()
            + Vec2::new(
                -self.board_width() - self.square_width() * 2.0,
                -self.square_height() * 2.0 - self.square_height() * 1.5 * index as f32,
            ))
        .extend(BOARD_Z_INDEX)
    }

    pub fn piece_count_counter_translation(&self, index: usize) -> Vec3 {
        Vec3::new(
            -self.board_width() + self.square_width(),
            -self.square_height() * 2.0 - self.square_height() * 1.5 * index as f32,
            BOARD_Z_INDEX,
        )
    }

    // right

    pub fn score_translation(&self) -> Vec3 {
        Vec3::new(
            self.board_width(),
            self.square_height() * 9.0,
            BOARD_Z_INDEX,
        )
    }

    pub fn level_translation(&self) -> Vec3 {
        Vec3::new(
            self.board_width(),
            -self.square_height() * 7.0,
            BOARD_Z_INDEX,
        )
    }

    pub fn game_mode_translation(&self) -> Vec3 {
        Vec3::new(
            self.board_width() * 1.4,
            self.square_height() * 2.0,
            BOARD_Z_INDEX,
        )
    }

    pub fn stopwatch_translation(&self) -> Vec3 {
        Vec3::new(
            self.board_width(),
            -self.square_height() * 9.0,
            BOARD_Z_INDEX,
        )
    }

    pub fn inputs_rect_size(&self) -> Vec2 {
        Vec2::splat(self.square_width() / 2.0)
    }

    pub fn inputs_circle_scale(&self) -> f32 {
        self.square_width() / 3.0
    }

    fn inputs_translation_offset(&self) -> Vec3 {
        Vec3::new(
            self.board_width(),
            -self.square_height() * 11.0,
            BOARD_Z_INDEX,
        )
    }

    pub fn inputs_button_center_translation(&self) -> Vec3 {
        self.inputs_translation_offset() + Vec3::new(self.square_width() * -2.0, 0.0, 0.0)
    }

    pub fn inputs_button_left_translation(&self) -> Vec3 {
        self.inputs_button_center_translation() + Vec3::new(self.square_width() * -0.5, 0.0, 0.0)
    }

    pub fn inputs_button_right_translation(&self) -> Vec3 {
        self.inputs_button_center_translation() + Vec3::new(self.square_width() * 0.5, 0.0, 0.0)
    }

    pub fn inputs_button_up_translation(&self) -> Vec3 {
        self.inputs_button_center_translation() + Vec3::new(0.0, self.square_width() * 0.5, 0.0)
    }

    pub fn inputs_button_down_translation(&self) -> Vec3 {
        self.inputs_button_center_translation() + Vec3::new(0.0, self.square_width() * -0.5, 0.0)
    }

    pub fn inputs_button_a_translation(&self) -> Vec3 {
        self.inputs_translation_offset() + Vec3::new(self.square_width() * 2.0, 0.0, 0.0)
    }

    pub fn inputs_button_b_translation(&self) -> Vec3 {
        self.inputs_translation_offset() + Vec3::new(self.square_width() * 1.0, 0.0, 0.0)
    }

    pub fn next_piece_square_size(&self, index: usize) -> Vec2 {
        match index {
            0 => self.square_size(),
            _ => self.square_size() / 2.0,
        }
    }

    fn next_piece_translation_offset(&self) -> Vec2 {
        Vec2::new(self.board_width() * 0.8, 0.0)
    }

    fn next_piece_translation_offset_for_index(&self, index: usize) -> Vec2 {
        Vec2::new(
            -self.square_width() * 4.0 + index as f32 * self.next_piece_square_size(index).x * 5.5,
            -self.square_height() * 4.0,
        )
    }

    pub fn next_piece_translation(&self, x: f32, y: f32, index: usize) -> Vec3 {
        match index {
            0 => {
                Vec2::new(x, y) * self.next_piece_square_size(index)
                    + self.next_piece_translation_offset()
            }
            _ => {
                Vec2::new(x, y) * self.next_piece_square_size(index)
                    + self.next_piece_translation_offset()
                    + self.next_piece_translation_offset_for_index(index)
            }
        }
        .extend(CURR_PIECE_Z_INDEX)
    }

    pub fn next_piece_slot_translation(&self, index: usize) -> Vec3 {
        match index {
            0 => self.next_piece_translation_offset(),
            _ => {
                self.next_piece_translation_offset()
                    + self.next_piece_translation_offset_for_index(index)
            }
        }
        .extend(BOARD_Z_INDEX)
    }

    pub fn next_piece_label_translation(&self) -> Vec3 {
        (self.next_piece_translation_offset() + Vec2::new(0.0, self.square_height() * 3.5))
            .extend(BOARD_Z_INDEX)
    }

    pub fn next_piece_slot_size(&self, index: usize) -> Vec2 {
        self.next_piece_square_size(index) * 5.0
    }

    pub fn next_piece_slot_background_translation(&self, index: usize) -> Vec3 {
        match index {
            0 => self.next_piece_translation_offset(),
            _ => {
                self.next_piece_translation_offset()
                    + self.next_piece_translation_offset_for_index(index)
            }
        }
        .extend(BOARD_BACKGROUND_Z_INDEX)
    }

    pub fn next_piece_slot_background_size(&self, index: usize) -> Vec2 {
        self.next_piece_slot_size(index) + self.border_size() * 2.0
    }
}

impl Default for GameTransform {
    fn default() -> Self {
        Self::new(1.0)
    }
}
