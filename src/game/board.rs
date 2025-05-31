use std::collections::VecDeque;

use rand::SeedableRng;

use super::{
    level::Level,
    next_piece_hint::NextPieceHint,
    piece::{Piece, Square},
    random::{PieceHistory, Random},
    scoring::Scoring,
    seed::Seed,
    seeding::Seeding,
    transition::Transition,
};

pub struct Board {
    start_level: Level,
    transition: Transition,
    scoring: Scoring,
    random: Random,
    seed: Seed,
    rng: rand_chacha::ChaCha20Rng,
    squares: Vec<Vec<Piece>>,
    curr_piece: Piece,
    curr_pos: (i32, i32),
    next_piece_hint: NextPieceHint,
    next_pieces: PieceHistory,
    lines: usize,
    score: usize,
    clear_lines_count: [usize; 4],
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

    pub fn new(
        start_level: Level,
        transition: Transition,
        scoring: Scoring,
        random: Random,
        seeding: Seeding,
        seed: Seed,
        next_piece_hint: NextPieceHint,
    ) -> Self {
        let seed = match seeding {
            Seeding::System => Seed::new(),
            Seeding::Custom => seed,
        };
        let mut rng = rand_chacha::ChaCha20Rng::from_seed(seed.into());
        let mut next_pieces = VecDeque::new();
        Self::gen_next_pieces(random, &mut rng, &mut next_pieces, next_piece_hint);

        let mut board = Self {
            start_level,
            transition,
            scoring,
            seed,
            random,
            rng,
            squares: vec![vec![Piece::default(); Self::BOARD_COLS]; Self::INTERNAL_BOARD_ROWS],
            curr_piece: Piece::X,
            curr_pos: (Self::BOARD_PIECE_START_X, Self::BOARD_PIECE_START_Y),
            next_piece_hint,
            next_pieces,
            lines: 0,
            score: 0,
            clear_lines_count: [0; 4],
            drought: 0,
            max_drought: 0,
            piece_count: [0; Piece::variant_len()],
        };

        // auto apply `drought` and `curr_piece`
        board.switch_to_next_piece();
        board
    }

    pub fn level(&self) -> Level {
        self.transition.transform(self.start_level, self.lines)
    }

    pub fn lines(&self) -> usize {
        self.lines
    }

    pub fn score(&self) -> usize {
        self.score
    }

    pub fn curr_level_score(&self, lines: usize) -> usize {
        assert!(lines >= 1 && lines <= 4);
        self.scoring.transform(lines, self.level())
    }

    pub fn burned_lines(&self) -> usize {
        self.lines - self.clear_lines_rate(4).0 * 4
    }

    pub fn clear_lines_rate(&self, lines: usize) -> (usize, Option<f32>) {
        assert!(lines >= 1 && lines <= 4);
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

    #[allow(dead_code)]
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

    pub fn clear_lines(&mut self) -> (Level, Level) {
        let rows = self.get_line_clear_rows();
        rows.iter().rev().for_each(|row| {
            self.squares.remove(*row);
        });

        let old_level = self.level();
        self.score += self.curr_level_score(rows.len());
        self.lines += rows.len();
        match rows.len() {
            1..=4 => self.clear_lines_count[rows.len() - 1] += 1,
            _ => (),
        }
        self.squares.resize(
            Self::INTERNAL_BOARD_ROWS,
            vec![Piece::default(); Self::BOARD_COLS],
        );

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

    pub fn next_pieces(&self) -> &PieceHistory {
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

    fn gen_next_pieces(
        random: Random,
        rng: &mut rand_chacha::ChaCha20Rng,
        history: &mut PieceHistory,
        next_piece_hint: NextPieceHint,
    ) {
        while history.len() <= next_piece_hint.count() {
            history.extend(random.gen_pieces(rng, history));
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new(
            Level(0),
            Transition::default(),
            Scoring::default(),
            Random::default(),
            Seeding::default(),
            Seed::default(),
            NextPieceHint::default(),
        )
    }
}
