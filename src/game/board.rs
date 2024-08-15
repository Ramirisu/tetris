use super::{
    level,
    piece::{Block, Piece, PieceShape},
};

const BOARD_ROWS: usize = 20;
pub const BOARD_COLS: usize = 10;
const BOARD_PIECE_START_X: i32 = (BOARD_COLS / 2) as i32;
const BOARD_PIECE_START_Y: i32 = (BOARD_ROWS - 1) as i32;

pub struct Board {
    pub blocks: Vec<Vec<bool>>,
    curr_piece: Piece,
    curr_translation: (i32, i32),
    next_piece: Piece,
    start_level: usize,
    pub lines: usize,
    pub score: usize,
    pub tetris_count: usize,
    pub drought: usize,
}

impl Board {
    pub fn new(start_level: usize) -> Self {
        let curr_piece = Piece::rand();
        let next_piece = curr_piece.rand_1h2r();
        Self {
            blocks: vec![vec![false; BOARD_COLS]; BOARD_ROWS],
            curr_piece,
            curr_translation: (BOARD_PIECE_START_X, BOARD_PIECE_START_Y),
            next_piece,
            start_level,
            lines: 0,
            score: 0,
            tetris_count: 0,
            drought: 0,
        }
    }

    pub fn level(&self) -> usize {
        level::level(self.start_level, self.lines)
    }

    pub fn burned(&self) -> usize {
        self.lines - self.tetris_count * 4
    }

    pub fn tetris_rate(&self) -> f32 {
        if self.lines == 0 {
            0.0
        } else {
            self.tetris_count as f32 * 4.0 / self.lines as f32
        }
    }

    pub fn get_line_clear_indexes(&self) -> Vec<usize> {
        let mut indexes = vec![];
        for index in 0..BOARD_ROWS {
            if self.blocks[index].iter().all(|blk| *blk) {
                indexes.push(index);
            }
        }

        indexes
    }

    pub fn lock_curr_piece(&mut self) {
        for blk in self.get_curr_piece_blocks() {
            self.blocks[blk.1 as usize][blk.0 as usize] = true;
        }
    }

    pub fn clear_lines(&mut self) {
        let indexes = self.get_line_clear_indexes();
        indexes.iter().rev().for_each(|index| {
            self.blocks.remove(*index);
        });

        if indexes.len() > 0 {
            if indexes.len() == 4 {
                self.tetris_count += 1;
            }
            self.score += self.lines_to_score(indexes.len());
            self.lines += indexes.len();
            self.blocks.resize(BOARD_ROWS, vec![false; BOARD_COLS]);
        }
    }

    pub fn switch_to_next_piece(&mut self) {
        if self.curr_piece.shape() == PieceShape::I {
            self.drought = 0;
        } else {
            self.drought += 1;
        }
        self.curr_piece = std::mem::replace(&mut self.next_piece, self.curr_piece.rand_1h2r());
        self.curr_translation = (BOARD_PIECE_START_X, BOARD_PIECE_START_Y);
    }

    pub fn get_curr_piece_blocks(&self) -> [Block; 4] {
        self.curr_piece.get_blocks().map(|blk| {
            Block(
                blk.0 + self.curr_translation.0,
                blk.1 + self.curr_translation.1,
            )
        })
    }

    pub fn get_next_piece_blocks(&self) -> [Block; 4] {
        self.next_piece.get_blocks()
    }

    pub fn is_left_movable(&self) -> bool {
        self.get_curr_piece_blocks().iter().all(|blk| {
            Self::is_inside_board(blk.0 - 1, blk.1)
                && (blk.1 as usize >= BOARD_ROWS
                    || !self.blocks[blk.1 as usize][(blk.0 - 1) as usize])
        })
    }

    pub fn is_right_movable(&self) -> bool {
        self.get_curr_piece_blocks().iter().all(|blk| {
            Self::is_inside_board(blk.0 + 1, blk.1)
                && (blk.1 as usize >= BOARD_ROWS
                    || !self.blocks[blk.1 as usize][(blk.0 + 1) as usize])
        })
    }

    pub fn is_curr_position_valid(&self) -> bool {
        self.get_curr_piece_blocks().iter().all(|blk| {
            Self::is_inside_board(blk.0, blk.1)
                && blk.1 < BOARD_ROWS as i32
                && !self.blocks[blk.1 as usize][blk.0 as usize]
        })
    }

    pub fn move_piece_down(&mut self) -> bool {
        let movable = self.get_curr_piece_blocks().iter().all(|blk| {
            Self::is_inside_board(blk.0, blk.1 - 1)
                && (blk.1 as usize >= BOARD_ROWS
                    || !self.blocks[(blk.1 - 1) as usize][blk.0 as usize])
        });

        if movable {
            self.curr_translation.1 -= 1;
        }

        movable
    }

    pub fn move_piece_left(&mut self) -> bool {
        let movable = self.is_left_movable();
        if movable {
            self.curr_translation.0 -= 1;
        }

        movable
    }

    pub fn move_piece_right(&mut self) -> bool {
        let movable = self.is_right_movable();
        if movable {
            self.curr_translation.0 += 1;
        }

        movable
    }

    pub fn rotate_piece_clockwise(&mut self) -> bool {
        self.curr_piece.rotate_clockwise();
        let rotatable = self.get_curr_piece_blocks().iter().all(|blk| {
            Self::is_inside_board(blk.0, blk.1)
                && (blk.1 as usize >= BOARD_ROWS || !self.blocks[blk.1 as usize][blk.0 as usize])
        });
        if !rotatable {
            self.curr_piece.rotate_counter_clockwise();
        }

        rotatable
    }

    pub fn rotate_piece_counter_clockwise(&mut self) -> bool {
        self.curr_piece.rotate_counter_clockwise();
        let rotatable = self.get_curr_piece_blocks().iter().all(|blk| {
            Self::is_inside_board(blk.0, blk.1)
                && (blk.1 as usize >= BOARD_ROWS || !self.blocks[blk.1 as usize][blk.0 as usize])
        });
        if !rotatable {
            self.curr_piece.rotate_clockwise();
        }

        rotatable
    }

    fn is_inside_board(x: i32, y: i32) -> bool {
        x >= 0 && x < BOARD_COLS as i32 && y >= 0
    }

    fn lines_to_score(&self, lines: usize) -> usize {
        (self.level() + 1)
            * match lines {
                1 => 40,
                2 => 100,
                3 => 300,
                4 => 1200,
                _ => panic!("can only clear lines between 1-4"),
            }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new(0)
    }
}
