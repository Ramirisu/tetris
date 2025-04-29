use std::collections::VecDeque;

use rand::SeedableRng;

use super::{
    piece::{Piece, Square},
    seed::Seed,
    seeding::Seeding,
    transition::Transition,
};

pub struct Board {
    start_level: usize,
    transition: Transition,
    seed: Seed,
    rng: rand_chacha::ChaCha20Rng,
    squares: Vec<Vec<Piece>>,
    curr_piece: Piece,
    curr_pos: (i32, i32),
    next_pieces: VecDeque<Piece>,
    lines: usize,
    score: usize,
    lines_clear: [usize; 4],
    drought: usize,
    max_drought: usize,
    piece_count: [usize; Piece::variant_len()],
}

impl Board {
    pub const BOARD_ROWS: usize = 20;
    pub const BOARD_COLS: usize = 10;
    pub const INTERNAL_BOARD_ROWS: usize = Self::BOARD_ROWS + 2;
    const BOARD_PIECE_START_X: i32 = (Self::BOARD_COLS / 2) as i32;
    const BOARD_PIECE_START_Y: i32 = (Self::BOARD_ROWS - 1) as i32;

    pub fn new(start_level: usize, transition: Transition, seeding: Seeding, seed: Seed) -> Self {
        let seed = match seeding {
            Seeding::System => Seed::new(),
            Seeding::Custom => seed,
        };
        let mut rng = rand_chacha::ChaCha20Rng::from_seed(seed.into());
        let next_pieces = (0..5).fold(VecDeque::new(), |mut accu, _| {
            accu.push_back(gen_piece_1h2r(&mut rng, &accu));
            accu
        });
        let mut board = Self {
            start_level,
            transition,
            seed,
            rng,
            squares: vec![vec![Piece::default(); Self::BOARD_COLS]; Self::INTERNAL_BOARD_ROWS],
            curr_piece: Piece::X,
            curr_pos: (Self::BOARD_PIECE_START_X, Self::BOARD_PIECE_START_Y),
            next_pieces,
            lines: 0,
            score: 0,
            lines_clear: [0; 4],
            drought: 0,
            max_drought: 0,
            piece_count: [0; Piece::variant_len()],
        };

        // auto apply `drought` and `curr_piece`
        board.switch_to_next_piece();
        board
    }

    pub fn level(&self) -> usize {
        self.transition
            .transform_level(self.start_level, self.lines)
    }

    pub fn lines(&self) -> usize {
        self.lines
    }

    pub fn score(&self) -> usize {
        self.score
    }

    pub fn burned_lines(&self) -> usize {
        self.lines - self.tetris_clear() * 4
    }

    pub fn single_clear(&self) -> usize {
        self.lines_clear[0]
    }

    pub fn double_clear(&self) -> usize {
        self.lines_clear[1]
    }

    pub fn triple_clear(&self) -> usize {
        self.lines_clear[2]
    }

    pub fn tetris_clear(&self) -> usize {
        self.lines_clear[3]
    }

    pub fn tetris_rate(&self) -> f32 {
        if self.lines == 0 {
            0.0
        } else {
            self.tetris_clear() as f32 * 4.0 / self.lines as f32
        }
    }

    pub fn drought(&self) -> usize {
        self.drought
    }

    pub fn max_drought(&self) -> usize {
        self.max_drought
    }

    pub fn seed(&self) -> Seed {
        self.seed
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
        for sqr in self.curr_piece_to_squares_with_pos() {
            self.squares[sqr.1 as usize][sqr.0 as usize] = self.curr_piece;
        }
    }

    pub fn clear_lines(&mut self) {
        let rows = self.get_line_clear_rows();
        rows.iter().rev().for_each(|row| {
            self.squares.remove(*row);
        });

        self.score += Self::transform_score(rows.len(), self.level());
        self.lines += rows.len();
        match rows.len() {
            1..=4 => self.lines_clear[rows.len() - 1] += 1,
            _ => (),
        }
        self.squares.resize(
            Self::INTERNAL_BOARD_ROWS,
            vec![Piece::default(); Self::BOARD_COLS],
        );
    }

    pub fn switch_to_next_piece(&mut self) {
        self.next_pieces
            .push_back(gen_piece_1h2r(&mut self.rng, &self.next_pieces));
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

    pub fn curr_piece(&self) -> &Piece {
        &self.curr_piece
    }

    pub fn curr_piece_to_squares_with_pos(&self) -> [Square; 4] {
        self.curr_piece
            .to_squares()
            .map(|sqr| Square(sqr.0 + self.curr_pos.0, sqr.1 + self.curr_pos.1))
    }

    pub fn next_pieces(&self) -> &VecDeque<Piece> {
        &self.next_pieces
    }

    pub fn is_left_movable(&self) -> bool {
        self.curr_piece_to_squares_with_pos().iter().all(|sqr| {
            let (x, y) = sqr.to_coordinate(-1, 0);
            Self::is_inside(x, y)
                && (y >= Self::BOARD_ROWS as i32 || self.get_square(x, y).is_placeholder())
        })
    }

    pub fn is_right_movable(&self) -> bool {
        self.curr_piece_to_squares_with_pos().iter().all(|sqr| {
            let (x, y) = sqr.to_coordinate(1, 0);
            Self::is_inside(x, y)
                && (y >= Self::BOARD_ROWS as i32 || self.get_square(x, y).is_placeholder())
        })
    }

    pub fn is_curr_position_valid(&self) -> bool {
        self.curr_piece_to_squares_with_pos().iter().all(|sqr| {
            let (x, y) = sqr.to_coordinate(0, 0);
            Self::is_inside(x, y)
                && y < Self::BOARD_ROWS as i32
                && self.get_square(x, y).is_placeholder()
        })
    }

    pub fn move_piece_down(&mut self) -> bool {
        let movable = self.curr_piece_to_squares_with_pos().iter().all(|sqr| {
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
        let rotatable = self.curr_piece_to_squares_with_pos().iter().all(|sqr| {
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
        let rotatable = self.curr_piece_to_squares_with_pos().iter().all(|sqr| {
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

    fn transform_score(lines: usize, level: usize) -> usize {
        (level + 1)
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
        Self::new(0, Transition::default(), Seeding::System, Seed::default())
    }
}

fn gen_piece<R: rand::Rng>(rng: &mut R) -> Piece {
    rng.random_range(0..(Piece::variant_len() - 1)).into()
}

fn gen_piece_1h2r<R: rand::Rng>(rng: &mut R, history: &VecDeque<Piece>) -> Piece {
    match history.back() {
        Some(piece) => {
            let index = rng.random_range(0..Piece::variant_len());
            if index + 1 != Piece::variant_len() && index != piece.variant_index() {
                index.into()
            } else {
                gen_piece(rng)
            }
        }
        None => gen_piece(rng),
    }
}
