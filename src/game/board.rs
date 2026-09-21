use std::collections::VecDeque;

use bevy::prelude::*;
use rand::{SeedableRng, rngs::ChaCha20Rng};

use crate::game::{
    level::Level,
    next_piece_hint::NextPieceHint,
    piece::{Piece, Square},
    random::{PieceHistory, Random},
    scoring::Scoring,
    seed::Seed,
    seeding::Seeding,
    transition::Transition,
};

#[derive(Clone, Copy)]
pub enum CurrPieceAction {
    MoveLeft,
    MoveRight,
    MoveDown,
    RotateClockwise,
    RotateCounterClockwise,
}

// Board coordinate:
//
// y=rows+ *       * | row padding
//         *       * | row padding
// y=rows  * start *
//         *       *
//         *       *
//         *       *
//         *       *
//         *       *
// y=0     *********
//        x=0    x=cols
pub struct Board {
    rows: usize,
    cols: usize,
    start_level: Level,
    transition: Transition,
    scoring: Scoring,
    random: Random,
    rng: ChaCha20Rng,
    next_piece_hint: NextPieceHint,
    squares: Vec<Vec<Piece>>,
    curr_piece: Piece,
    curr_piece_pos_x: i32,
    curr_piece_pos_y: i32,
    next_pieces: PieceHistory,
    score: usize,
    lines: usize,
    clear_lines_count: [usize; 4],
    drought: usize,
    max_drought: usize,
    piece_count: [usize; Piece::variant_len()],
}

impl Board {
    const ROWS_PADDING: usize = 2;

    pub fn new(
        rows: usize,
        cols: usize,
        start_level: Level,
        transition: Transition,
        scoring: Scoring,
        random: Random,
        seeding: Seeding,
        seed: Seed,
        next_piece_hint: NextPieceHint,
    ) -> Self {
        if rows == 0 || cols == 0 {
            panic!("rows and cols must not be zero");
        }

        let seed = match seeding {
            Seeding::System => Seed::new(),
            Seeding::Custom => seed,
        };

        let mut board = Self {
            rows,
            cols,
            start_level,
            transition,
            scoring,
            random,
            rng: ChaCha20Rng::from_seed(seed.into()),
            next_piece_hint,
            squares: vec![vec![Piece::default(); cols]; Self::padded_rows(rows)],
            curr_piece: Piece::default(),
            curr_piece_pos_x: 0,
            curr_piece_pos_y: 0,
            next_pieces: VecDeque::new(),
            score: 0,
            lines: 0,
            clear_lines_count: [0; 4],
            drought: 0,
            max_drought: 0,
            piece_count: [0; Piece::variant_len()],
        };

        board.switch_to_next_piece();
        board
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn padded_rows(rows: usize) -> usize {
        rows + Self::ROWS_PADDING
    }

    pub fn square(&self, x: usize, y: usize) -> Piece {
        self.squares[y][x]
    }

    pub fn level(&self) -> Level {
        self.transition.transform(self.start_level, self.lines)
    }

    pub fn curr_level_score(&self, lines: usize) -> usize {
        self.scoring.transform(lines, self.level())
    }

    pub fn lines(&self) -> usize {
        self.lines
    }

    pub fn score(&self) -> usize {
        self.score
    }

    pub fn burned_lines(&self) -> usize {
        self.lines - self.clear_lines_rate(4).0 * 4
    }

    pub fn clear_lines_rate(&self, lines: usize) -> (usize, Option<f32>) {
        let count = self.clear_lines_count[lines - 1];

        if self.lines == 0 {
            (count, None)
        } else {
            (count, Some((count * lines) as f32 / self.lines as f32))
        }
    }

    pub fn drought(&self) -> usize {
        self.drought
    }

    #[allow(dead_code)]
    pub fn max_drought(&self) -> usize {
        self.max_drought
    }

    pub fn piece_count(&self, piece: Piece) -> usize {
        self.piece_count[piece.variant_index()]
    }

    pub fn curr_piece(&self) -> &Piece {
        &self.curr_piece
    }

    pub fn curr_piece_to_squares_with_pos(&self) -> [Square; 4] {
        Self::piece_to_squares(
            self.curr_piece,
            self.curr_piece_pos_x,
            self.curr_piece_pos_y,
        )
    }

    pub fn is_curr_piece_pos_valid(&self) -> bool {
        self.curr_piece_to_squares_with_pos().iter().all(|sqr| {
            self.is_inside_board(sqr.0, sqr.1)
                && self.square(sqr.0 as usize, sqr.1 as usize).is_placeholder()
        })
    }

    pub fn next_pieces(&self) -> &PieceHistory {
        &self.next_pieces
    }

    pub fn can_apply_action(&self, action: CurrPieceAction) -> bool {
        self.apply_action_impl(action).is_some()
    }

    pub fn try_apply_action(&mut self, action: CurrPieceAction) -> bool {
        self.apply_action_impl(action)
            .map(|(x, y, piece)| {
                self.curr_piece_pos_x = x;
                self.curr_piece_pos_y = y;
                self.curr_piece = piece;
                true
            })
            .unwrap_or(false)
    }

    pub fn lock_curr_piece(&mut self) {
        for sqr in Self::piece_to_squares(
            self.curr_piece,
            self.curr_piece_pos_x,
            self.curr_piece_pos_y,
        ) {
            self.squares[sqr.1 as usize][sqr.0 as usize] = self.curr_piece
        }
    }

    pub fn get_filled_lines(&self) -> Vec<usize> {
        let mut rows = Vec::new();
        for (index, row) in self.squares.iter().enumerate() {
            if row.iter().all(|sqr| !sqr.is_placeholder()) {
                rows.push(index);
            }
        }

        rows
    }

    pub fn clear_filled_lines(&mut self) -> (Level, Level) {
        let rows = self.get_filled_lines();
        if rows.len() == 0 {
            return (self.level(), self.level());
        }

        let mut curr_row = rows[0];
        for row in curr_row..self.squares.len() {
            if rows.binary_search(&row).is_ok() {
                for square in &mut self.squares[row] {
                    *square = Piece::X
                }
                continue;
            }
            if curr_row != row {
                for index in 0..self.cols {
                    self.squares[curr_row][index] = self.squares[row][index];
                    self.squares[row][index] = Piece::X;
                }
                curr_row += 1;
            }
        }

        let old_level = self.level();
        self.score += self.curr_level_score(rows.len());
        self.lines += rows.len();
        match rows.len() {
            1..=4 => self.clear_lines_count[rows.len() - 1] += 1,
            _ => (),
        }

        (self.level(), old_level)
    }

    pub fn switch_to_next_piece(&mut self) {
        Self::gen_next_pieces(
            self.random,
            &mut self.rng,
            &mut self.next_pieces,
            self.next_piece_hint,
        );
        self.curr_piece = self.next_pieces.pop_front().unwrap();

        self.curr_piece_pos_x = (self.cols / 2) as i32;
        self.curr_piece_pos_y = (self.rows - 1) as i32;
        self.piece_count[self.curr_piece.variant_index()] += 1;
        match self.curr_piece {
            Piece::I(_) => self.drought = 0,
            _ => {
                self.drought += 1;
                self.max_drought = self.max_drought.max(self.drought);
            }
        }
    }

    fn is_inside_board(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.cols as i32 && y >= 0 && y < Self::padded_rows(self.rows) as i32
    }

    fn piece_to_squares(piece: Piece, x: i32, y: i32) -> [Square; 4] {
        piece.to_squares().map(|sqr| Square(sqr.0 + x, sqr.1 + y))
    }

    fn apply_action_impl(&self, action: CurrPieceAction) -> Option<(i32, i32, Piece)> {
        let new_pos_x = match action {
            CurrPieceAction::MoveLeft => self.curr_piece_pos_x - 1,
            CurrPieceAction::MoveRight => self.curr_piece_pos_x + 1,
            _ => self.curr_piece_pos_x,
        };
        let new_pos_y = match action {
            CurrPieceAction::MoveDown => self.curr_piece_pos_y - 1,
            _ => self.curr_piece_pos_y,
        };

        let new_piece = {
            let mut piece = self.curr_piece;
            match action {
                CurrPieceAction::RotateClockwise => piece.rotate_clockwise(),
                CurrPieceAction::RotateCounterClockwise => piece.rotate_counterclockwise(),
                _ => (),
            };

            piece
        };

        Self::piece_to_squares(new_piece, new_pos_x, new_pos_y)
            .iter()
            .all(|sqr| {
                self.is_inside_board(sqr.0, sqr.1)
                    && self.square(sqr.0 as usize, sqr.1 as usize).is_placeholder()
            })
            .then_some((new_pos_x, new_pos_y, new_piece))
    }

    fn gen_next_pieces(
        random: Random,
        rng: &mut ChaCha20Rng,
        history: &mut PieceHistory,
        next_piece_hint: NextPieceHint,
    ) {
        while history.len() <= next_piece_hint.count() {
            history.extend(random.gen_pieces(rng, history));
        }
    }
}
