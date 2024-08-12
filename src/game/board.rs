use super::piece::{Block, Piece};

const BOARD_ROWS: usize = 20;
const BOARD_COLS: usize = 10;
const BOARD_PIECE_START_X: i32 = (BOARD_COLS / 2) as i32;
const BOARD_PIECE_START_Y: i32 = (BOARD_ROWS - 1) as i32;

pub struct Board {
    pub blocks: Vec<Vec<bool>>,
    curr_piece: Piece,
    curr_translation: (i32, i32),
}

impl Board {
    pub fn new() -> Self {
        Self {
            blocks: vec![vec![false; BOARD_COLS]; BOARD_ROWS],
            curr_piece: Piece::rand(),
            curr_translation: (BOARD_PIECE_START_X, BOARD_PIECE_START_Y),
        }
    }

    pub fn switch_to_next_piece(&mut self) {
        self.curr_piece = Piece::rand();
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

    pub fn move_curr_piece_down(&mut self) -> bool {
        let movable = self.get_curr_piece_blocks().iter().all(|blk| {
            blk.1 > 0
                && (blk.1 as usize > BOARD_ROWS
                    || !self.blocks[(blk.1 - 1) as usize][blk.0 as usize])
        });

        if movable {
            self.curr_translation.1 -= 1;
        }

        movable
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}
