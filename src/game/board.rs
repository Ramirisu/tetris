use std::collections::VecDeque;

use super::{
    piece::{Piece, Square},
    seed::Seed,
    transition::Transition,
};

pub struct Board {
    start_level: usize,
    transition: Transition,
    seed: Seed,
    squares: Vec<Vec<Piece>>,
    curr_piece: Piece,
    curr_pos: (i32, i32),
    next_pieces: VecDeque<Piece>,
    lines: usize,
    score: usize,
    single: usize,
    double: usize,
    triple: usize,
    tetris: usize,
    drought: usize,
    max_drought: usize,
    piece_count: [usize; Piece::variant_len()],
}

impl Board {
    pub const BOARD_ROWS: usize = 20;
    pub const BOARD_COLS: usize = 10;
    const INTERNAL_BOARD_ROWS: usize = Self::BOARD_ROWS + 2;
    const BOARD_PIECE_START_X: i32 = (Self::BOARD_COLS / 2) as i32;
    const BOARD_PIECE_START_Y: i32 = (Self::BOARD_ROWS - 1) as i32;

    pub fn new(start_level: usize, transition: Transition) -> Self {
        let seed = Seed::System;
        let next_pieces = (0..5).fold(VecDeque::new(), |mut accu, _| {
            accu.push_back(seed.gen_1h2r(&accu));
            accu
        });
        let mut board = Self {
            start_level,
            transition,
            seed,
            squares: vec![vec![Piece::default(); Self::BOARD_COLS]; Self::BOARD_ROWS],
            curr_piece: Piece::X,
            curr_pos: (Self::BOARD_PIECE_START_X, Self::BOARD_PIECE_START_Y),
            next_pieces,
            lines: 0,
            score: 0,
            single: 0,
            double: 0,
            triple: 0,
            tetris: 0,
            drought: 0,
            max_drought: 0,
            piece_count: [0; Piece::variant_len()],
        };

        // auto apply `drought` and `curr_piece`
        board.switch_to_next_piece();
        board
    }

    pub fn level(&self) -> usize {
        self.transition.get_level(self.start_level, self.lines)
    }

    pub fn lines(&self) -> usize {
        self.lines
    }

    pub fn score(&self) -> usize {
        self.score
    }

    pub fn burned_lines(&self) -> usize {
        self.lines - self.tetris * 4
    }

    pub fn single(&self) -> usize {
        self.single
    }

    pub fn double(&self) -> usize {
        self.double
    }

    pub fn triple(&self) -> usize {
        self.triple
    }

    pub fn tetris(&self) -> usize {
        self.tetris
    }

    pub fn tetris_rate(&self) -> f32 {
        if self.lines == 0 {
            0.0
        } else {
            self.tetris as f32 * 4.0 / self.lines as f32
        }
    }

    pub fn drought(&self) -> usize {
        self.drought
    }

    pub fn max_drought(&self) -> usize {
        self.max_drought
    }

    pub fn get_square(&self, x: i32, y: i32) -> Piece {
        self.squares[y as usize][x as usize]
    }

    pub fn get_line_clear_rows(&self) -> Vec<usize> {
        let mut rows = vec![];
        for row in 0..Self::BOARD_ROWS {
            if self.squares[row].iter().all(|sqr| !sqr.is_placeholder()) {
                rows.push(row);
            }
        }

        rows
    }

    pub fn get_piece_count(&self, piece: Piece) -> usize {
        self.piece_count[piece.variant_index()]
    }

    pub fn lock_curr_piece(&mut self) {
        for sqr in self.get_curr_piece_squares() {
            self.squares[sqr.1 as usize][sqr.0 as usize] = self.curr_piece;
        }
    }

    pub fn clear_lines(&mut self) {
        let rows = self.get_line_clear_rows();
        rows.iter().rev().for_each(|row| {
            self.squares.remove(*row);
        });

        self.score += Self::get_score(rows.len(), self.level());
        self.lines += rows.len();
        match rows.len() {
            1 => self.single += 1,
            2 => self.double += 1,
            3 => self.triple += 1,
            4 => self.tetris += 1,
            _ => (),
        }
        self.squares
            .resize(Self::BOARD_ROWS, vec![Piece::default(); Self::BOARD_COLS]);
    }

    pub fn switch_to_next_piece(&mut self) {
        self.next_pieces
            .push_back(self.seed.gen_1h2r(&self.next_pieces));
        self.curr_piece = self.next_pieces.pop_front().unwrap();

        self.curr_pos = (Self::BOARD_PIECE_START_X, Self::BOARD_PIECE_START_Y);
        self.piece_count[self.curr_piece.variant_index()] += 1;
        match self.curr_piece {
            Piece::I(_) => self.drought = 0,
            _ => {
                self.drought += 1;
                self.max_drought = self.max_drought.max(self.drought);
            }
        }
    }

    pub fn get_curr_piece(&self) -> Piece {
        self.curr_piece
    }

    pub fn get_curr_piece_squares(&self) -> [Square; 4] {
        self.curr_piece
            .get_squares()
            .map(|sqr| Square(sqr.0 + self.curr_pos.0, sqr.1 + self.curr_pos.1))
    }

    pub fn get_next_pieces(&self) -> &VecDeque<Piece> {
        &self.next_pieces
    }

    pub fn is_left_movable(&self) -> bool {
        self.get_curr_piece_squares().iter().all(|sqr| {
            let (x, y) = sqr.to_coordinate(-1, 0);
            Self::is_inside(x, y)
                && (y >= Self::BOARD_ROWS as i32 || self.get_square(x, y).is_placeholder())
        })
    }

    pub fn is_right_movable(&self) -> bool {
        self.get_curr_piece_squares().iter().all(|sqr| {
            let (x, y) = sqr.to_coordinate(1, 0);
            Self::is_inside(x, y)
                && (y >= Self::BOARD_ROWS as i32 || self.get_square(x, y).is_placeholder())
        })
    }

    pub fn is_curr_position_valid(&self) -> bool {
        self.get_curr_piece_squares().iter().all(|sqr| {
            let (x, y) = sqr.to_coordinate(0, 0);
            Self::is_inside(x, y)
                && y < Self::BOARD_ROWS as i32
                && self.get_square(x, y).is_placeholder()
        })
    }

    pub fn move_piece_down(&mut self) -> bool {
        let movable = self.get_curr_piece_squares().iter().all(|sqr| {
            let (x, y) = sqr.to_coordinate(0, -1);
            Self::is_inside(x, y)
                && (y >= Self::BOARD_ROWS as i32 || self.get_square(x, y).is_placeholder())
        });

        if movable {
            self.curr_pos.1 -= 1;
        }

        movable
    }

    pub fn move_piece_left(&mut self) -> bool {
        let movable = self.is_left_movable();
        if movable {
            self.curr_pos.0 -= 1;
        }

        movable
    }

    pub fn move_piece_right(&mut self) -> bool {
        let movable = self.is_right_movable();
        if movable {
            self.curr_pos.0 += 1;
        }

        movable
    }

    pub fn rotate_piece_clockwise(&mut self) -> bool {
        self.curr_piece.rotate_clockwise();
        let rotatable = self.get_curr_piece_squares().iter().all(|sqr| {
            let (x, y) = sqr.to_coordinate(0, 0);
            Self::is_inside(x, y)
                && (y >= Self::BOARD_ROWS as i32 || self.get_square(x, y).is_placeholder())
        });
        if !rotatable {
            self.curr_piece.rotate_counterclockwise();
        }

        rotatable
    }

    pub fn rotate_piece_counter_clockwise(&mut self) -> bool {
        self.curr_piece.rotate_counterclockwise();
        let rotatable = self.get_curr_piece_squares().iter().all(|sqr| {
            let (x, y) = sqr.to_coordinate(0, 0);
            Self::is_inside(x, y)
                && (y >= Self::BOARD_ROWS as i32 || self.get_square(x, y).is_placeholder())
        });
        if !rotatable {
            self.curr_piece.rotate_clockwise();
        }

        rotatable
    }

    fn is_inside(x: i32, y: i32) -> bool {
        x >= 0 && x < Self::BOARD_COLS as i32 && y >= 0 && y < Self::INTERNAL_BOARD_ROWS as i32
    }

    fn get_score(lines: usize, level: usize) -> usize {
        (level + 1)
            * match lines {
                0 => 0,
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
        Self::new(0, Transition::default())
    }
}
