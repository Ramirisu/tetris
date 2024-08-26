use rand::Rng;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Block(pub i32, pub i32);

const PIECE_SHAPE_T: [[Block; 4]; 4] = [
    [Block(0, -1), Block(-1, 0), Block(0, 0), Block(1, 0)],
    [Block(0, -1), Block(-1, 0), Block(0, 0), Block(0, 1)],
    [Block(-1, 0), Block(0, 0), Block(1, 0), Block(0, 1)],
    [Block(0, -1), Block(0, 0), Block(1, 0), Block(0, 1)],
];

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

const PIECE_SHAPE_O: [[Block; 4]; 1] = [[Block(-1, -1), Block(0, -1), Block(-1, 0), Block(0, 0)]];

const PIECE_SHAPE_S: [[Block; 4]; 2] = [
    [Block(-1, -1), Block(0, -1), Block(0, 0), Block(1, 0)],
    [Block(1, -1), Block(0, 0), Block(1, 0), Block(0, 1)],
];

const PIECE_SHAPE_Z: [[Block; 4]; 2] = [
    [Block(0, -1), Block(1, -1), Block(-1, 0), Block(0, 0)],
    [Block(0, -1), Block(0, 0), Block(1, 0), Block(1, 1)],
];

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PieceShape {
    T,
    J,
    Z,
    O,
    S,
    L,
    I,
}

impl PieceShape {
    pub fn state_len(&self) -> usize {
        match self {
            PieceShape::T => PIECE_SHAPE_T.len(),
            PieceShape::J => PIECE_SHAPE_J.len(),
            PieceShape::Z => PIECE_SHAPE_Z.len(),
            PieceShape::O => PIECE_SHAPE_O.len(),
            PieceShape::S => PIECE_SHAPE_S.len(),
            PieceShape::L => PIECE_SHAPE_L.len(),
            PieceShape::I => PIECE_SHAPE_I.len(),
        }
    }

    pub fn iter() -> std::slice::Iter<'static, PieceShape> {
        const SHAPES: [PieceShape; 7] = [
            PieceShape::T,
            PieceShape::J,
            PieceShape::Z,
            PieceShape::O,
            PieceShape::S,
            PieceShape::L,
            PieceShape::I,
        ];
        SHAPES.iter()
    }
}

impl From<usize> for PieceShape {
    fn from(value: usize) -> Self {
        match value % 7 {
            0 => PieceShape::T,
            1 => PieceShape::J,
            2 => PieceShape::Z,
            3 => PieceShape::O,
            4 => PieceShape::S,
            5 => PieceShape::L,
            _ => PieceShape::I,
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
        Self::new(rand::thread_rng().gen::<usize>().into())
    }

    pub fn rand_1h2r(&self) -> Piece {
        let shape = rand::thread_rng().gen_range(0..8);
        if shape != self.shape as usize {
            Self::new(shape.into())
        } else {
            Self::rand()
        }
    }

    pub fn shape(&self) -> PieceShape {
        self.shape
    }

    pub fn get_blocks(&self) -> [Block; 4] {
        match self.shape {
            PieceShape::T => PIECE_SHAPE_T[self.state],
            PieceShape::J => PIECE_SHAPE_J[self.state],
            PieceShape::Z => PIECE_SHAPE_Z[self.state],
            PieceShape::O => PIECE_SHAPE_O[self.state],
            PieceShape::S => PIECE_SHAPE_S[self.state],
            PieceShape::L => PIECE_SHAPE_L[self.state],
            PieceShape::I => PIECE_SHAPE_I[self.state],
        }
    }

    pub fn rotate_clockwise(&mut self) {
        self.state = (self.state + 1) % self.shape.state_len();
    }

    pub fn rotate_counter_clockwise(&mut self) {
        self.state = (self.state + self.shape.state_len() - 1) % self.shape.state_len();
    }
}
