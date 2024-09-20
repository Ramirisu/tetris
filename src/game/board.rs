use super::{
    piece::{Piece, PieceShape, Square},
    score::get_score,
    transition::Transition,
};

pub struct Board {
    start_level: usize,
    transition: Transition,
    squares: Vec<Vec<PieceShape>>,
    curr_piece: Piece,
    curr_translation: (i32, i32),
    next_piece: Piece,
    lines: usize,
    score: usize,
    single: usize,
    double: usize,
    triple: usize,
    tetris: usize,
    drought: usize,
    max_drought: usize,
    piece_count: [usize; PieceShape::variant_size()],
}

impl Board {
    pub const BOARD_ROWS: usize = 20;
    pub const BOARD_COLS: usize = 10;
    const INTERNAL_BOARD_ROWS: usize = Self::BOARD_ROWS + 2;
    const BOARD_PIECE_START_X: i32 = (Self::BOARD_COLS / 2) as i32;
    const BOARD_PIECE_START_Y: i32 = (Self::BOARD_ROWS - 1) as i32;

    pub fn new(start_level: usize, transition: Transition) -> Self {
        let mut board = Self {
            start_level,
            transition,
            squares: vec![vec![PieceShape::default(); Self::BOARD_COLS]; Self::BOARD_ROWS],
            curr_piece: Piece::rand(),
            curr_translation: (Self::BOARD_PIECE_START_X, Self::BOARD_PIECE_START_Y),
            next_piece: Piece::rand(),
            lines: 0,
            score: 0,
            single: 0,
            double: 0,
            triple: 0,
            tetris: 0,
            drought: 0,
            max_drought: 0,
            piece_count: [0; PieceShape::variant_size()],
        };

        // auto apply `drought` and `rand_1h2r`
        board.switch_to_next_piece();
        board
    }

    pub fn start_level(&self) -> usize {
        self.start_level
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

    pub fn get_square(&self, x: i32, y: i32) -> PieceShape {
        self.squares[y as usize][x as usize]
    }

    pub fn get_line_clear_indexes(&self) -> Vec<usize> {
        let mut indexes = vec![];
        for index in 0..Self::BOARD_ROWS {
            if self.squares[index].iter().all(|sqr| !sqr.is_placeholder()) {
                indexes.push(index);
            }
        }

        indexes
    }

    pub fn get_piece_count(&self, shape: PieceShape) -> usize {
        self.piece_count[shape as usize]
    }

    pub fn lock_curr_piece(&mut self) {
        for sqr in self.get_curr_piece_squares() {
            self.squares[sqr.1 as usize][sqr.0 as usize] = self.curr_piece.shape();
        }
    }

    pub fn clear_lines(&mut self) {
        let indexes = self.get_line_clear_indexes();
        indexes.iter().rev().for_each(|index| {
            self.squares.remove(*index);
        });

        self.score += get_score(indexes.len(), self.level());
        self.lines += indexes.len();
        match indexes.len() {
            1 => self.single += 1,
            2 => self.double += 1,
            3 => self.triple += 1,
            4 => self.tetris += 1,
            _ => (),
        }
        self.squares.resize(
            Self::BOARD_ROWS,
            vec![PieceShape::default(); Self::BOARD_COLS],
        );
    }

    pub fn switch_to_next_piece(&mut self) {
        self.curr_piece = std::mem::replace(&mut self.next_piece, self.curr_piece.rand_1h2r());
        self.curr_translation = (Self::BOARD_PIECE_START_X, Self::BOARD_PIECE_START_Y);
        if self.curr_piece.shape() == PieceShape::I {
            self.drought = 0;
        } else {
            self.drought += 1;
            self.max_drought = self.max_drought.max(self.drought);
        }
        self.piece_count[self.curr_piece.shape() as usize] += 1;
    }

    pub fn get_curr_piece(&self) -> Piece {
        self.curr_piece
    }

    pub fn get_curr_piece_squares(&self) -> [Square; 4] {
        self.curr_piece.to_squares().map(|sqr| {
            Square(
                sqr.0 + self.curr_translation.0,
                sqr.1 + self.curr_translation.1,
            )
        })
    }

    pub fn get_next_piece(&self) -> Piece {
        self.next_piece
    }

    pub fn is_left_movable(&self) -> bool {
        self.get_curr_piece_squares().iter().all(|sqr| {
            let x = sqr.0 - 1;
            let y = sqr.1;
            Self::is_inside(x, y)
                && (y >= Self::BOARD_ROWS as i32 || self.get_square(x, y).is_placeholder())
        })
    }

    pub fn is_right_movable(&self) -> bool {
        self.get_curr_piece_squares().iter().all(|sqr| {
            let x = sqr.0 + 1;
            let y = sqr.1;
            Self::is_inside(x, y)
                && (y >= Self::BOARD_ROWS as i32 || self.get_square(x, y).is_placeholder())
        })
    }

    pub fn is_curr_position_valid(&self) -> bool {
        self.get_curr_piece_squares().iter().all(|sqr| {
            let x = sqr.0;
            let y = sqr.1;
            Self::is_inside(x, y)
                && y < Self::BOARD_ROWS as i32
                && self.get_square(x, y).is_placeholder()
        })
    }

    pub fn move_piece_down(&mut self) -> bool {
        let movable = self.get_curr_piece_squares().iter().all(|sqr| {
            let x = sqr.0;
            let y = sqr.1 - 1;
            Self::is_inside(x, y)
                && (y >= Self::BOARD_ROWS as i32 || self.get_square(x, y).is_placeholder())
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
        self.curr_piece.next_state();
        let rotatable = self.get_curr_piece_squares().iter().all(|sqr| {
            let x = sqr.0;
            let y = sqr.1;
            Self::is_inside(x, y)
                && (y >= Self::BOARD_ROWS as i32 || self.get_square(x, y).is_placeholder())
        });
        if !rotatable {
            self.curr_piece.prev_state();
        }

        rotatable
    }

    pub fn rotate_piece_counter_clockwise(&mut self) -> bool {
        self.curr_piece.prev_state();
        let rotatable = self.get_curr_piece_squares().iter().all(|sqr| {
            let x = sqr.0;
            let y = sqr.1;
            Self::is_inside(x, y)
                && (y >= Self::BOARD_ROWS as i32 || self.get_square(x, y).is_placeholder())
        });
        if !rotatable {
            self.curr_piece.next_state();
        }

        rotatable
    }

    fn is_inside(x: i32, y: i32) -> bool {
        x >= 0 && x < Self::BOARD_COLS as i32 && y >= 0 && y < Self::INTERNAL_BOARD_ROWS as i32
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new(0, Transition::default())
    }
}
