use rand::Rng;

#[derive(Clone, Copy)]
pub struct Block(pub i32, pub i32);

const PIECE_SHAPE_I: [[Block; 4]; 2] = [
    [Block(-2, 0), Block(-1, 0), Block(0, 0), Block(1, 0)],
    [Block(0, -1), Block(0, 0), Block(0, 1), Block(0, 2)],
];

const PIECE_SHAPE_J: [[Block; 4]; 4] = [
    [Block(-1, 0), Block(0, 0), Block(1, 0), Block(1, -1)],
    [Block(-1, -1), Block(0, -1), Block(0, 0), Block(0, 1)],
    [Block(-1, 0), Block(0, 0), Block(1, 0), Block(-1, 1)],
    [Block(0, -1), Block(0, 0), Block(0, 1), Block(1, 1)],
];

const PIECE_SHAPE_L: [[Block; 4]; 4] = [
    [Block(-1, -1), Block(-1, 0), Block(0, 0), Block(1, 0)],
    [Block(0, -1), Block(0, 0), Block(-1, 1), Block(0, 1)],
    [Block(-1, 0), Block(0, 0), Block(1, 0), Block(1, 1)],
    [Block(0, -1), Block(1, -1), Block(0, 0), Block(0, 1)],
];

const PIECE_SHAPE_O: [[Block; 4]; 1] = [[Block(-1, 0), Block(0, 0), Block(-1, 1), Block(0, 1)]];

const PIECE_SHAPE_S: [[Block; 4]; 2] = [
    [Block(-1, -1), Block(0, -1), Block(0, 0), Block(1, 0)],
    [Block(1, -1), Block(0, 0), Block(1, 0), Block(0, 1)],
];

const PIECE_SHAPE_T: [[Block; 4]; 4] = [
    [Block(0, -1), Block(-1, 0), Block(0, 0), Block(1, 0)],
    [Block(0, -1), Block(-1, 0), Block(0, 0), Block(0, 1)],
    [Block(-1, 0), Block(0, 0), Block(1, 0), Block(0, 1)],
    [Block(0, -1), Block(0, 0), Block(1, 0), Block(0, 1)],
];

const PIECE_SHAPE_Z: [[Block; 4]; 2] = [
    [Block(0, -1), Block(1, -1), Block(-1, 0), Block(0, 0)],
    [Block(0, -1), Block(0, 0), Block(1, 0), Block(1, 1)],
];

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PieceShape {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl PieceShape {
    pub fn state_len(&self) -> usize {
        match self {
            PieceShape::I => PIECE_SHAPE_I.len(),
            PieceShape::J => PIECE_SHAPE_J.len(),
            PieceShape::L => PIECE_SHAPE_L.len(),
            PieceShape::O => PIECE_SHAPE_O.len(),
            PieceShape::S => PIECE_SHAPE_S.len(),
            PieceShape::T => PIECE_SHAPE_T.len(),
            PieceShape::Z => PIECE_SHAPE_Z.len(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Piece {
    shape: PieceShape,
    state: usize,
}

impl Piece {
    pub fn new(shape: PieceShape) -> Self {
        Self { shape, state: 0 }
    }

    pub fn rand() -> Self {
        Self::new(match rand::thread_rng().gen::<usize>() % 7 {
            0 => PieceShape::I,
            1 => PieceShape::J,
            2 => PieceShape::L,
            3 => PieceShape::O,
            4 => PieceShape::S,
            5 => PieceShape::T,
            _ => PieceShape::Z,
        })
    }

    pub fn shape(&self) -> PieceShape {
        self.shape
    }

    pub fn get_blocks(&self) -> [Block; 4] {
        match self.shape {
            PieceShape::I => PIECE_SHAPE_I[self.state],
            PieceShape::J => PIECE_SHAPE_J[self.state],
            PieceShape::L => PIECE_SHAPE_L[self.state],
            PieceShape::O => PIECE_SHAPE_O[self.state],
            PieceShape::S => PIECE_SHAPE_S[self.state],
            PieceShape::T => PIECE_SHAPE_T[self.state],
            PieceShape::Z => PIECE_SHAPE_Z[self.state],
        }
    }

    pub fn rotate_clockwise(&mut self) {
        self.state = (self.state + 1) % self.shape.state_len();
    }

    pub fn rotate_counter_clockwise(&mut self) {
        self.state = (self.state + self.shape.state_len() - 1) % self.shape.state_len();
    }
}
