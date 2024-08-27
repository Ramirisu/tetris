use rand::Rng;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Square(pub i32, pub i32);

const PIECE_SHAPE_T: &[[Square; 4]; 4] = &[
    [Square(0, -1), Square(-1, 0), Square(0, 0), Square(1, 0)],
    [Square(0, -1), Square(-1, 0), Square(0, 0), Square(0, 1)],
    [Square(-1, 0), Square(0, 0), Square(1, 0), Square(0, 1)],
    [Square(0, -1), Square(0, 0), Square(1, 0), Square(0, 1)],
];

const PIECE_SHAPE_I: &[[Square; 4]; 2] = &[
    [Square(-2, 0), Square(-1, 0), Square(0, 0), Square(1, 0)],
    [Square(0, -1), Square(0, 0), Square(0, 1), Square(0, 2)],
];

const PIECE_SHAPE_J: &[[Square; 4]; 4] = &[
    [Square(-1, 0), Square(0, 0), Square(1, 0), Square(1, -1)],
    [Square(-1, -1), Square(0, -1), Square(0, 0), Square(0, 1)],
    [Square(-1, 0), Square(0, 0), Square(1, 0), Square(-1, 1)],
    [Square(0, -1), Square(0, 0), Square(0, 1), Square(1, 1)],
];

const PIECE_SHAPE_L: &[[Square; 4]; 4] = &[
    [Square(-1, -1), Square(-1, 0), Square(0, 0), Square(1, 0)],
    [Square(0, -1), Square(0, 0), Square(-1, 1), Square(0, 1)],
    [Square(-1, 0), Square(0, 0), Square(1, 0), Square(1, 1)],
    [Square(0, -1), Square(1, -1), Square(0, 0), Square(0, 1)],
];

const PIECE_SHAPE_O: &[[Square; 4]; 1] =
    &[[Square(-1, -1), Square(0, -1), Square(-1, 0), Square(0, 0)]];

const PIECE_SHAPE_S: &[[Square; 4]; 2] = &[
    [Square(-1, -1), Square(0, -1), Square(0, 0), Square(1, 0)],
    [Square(1, -1), Square(0, 0), Square(1, 0), Square(0, 1)],
];

const PIECE_SHAPE_Z: &[[Square; 4]; 2] = &[
    [Square(0, -1), Square(1, -1), Square(-1, 0), Square(0, 0)],
    [Square(0, -1), Square(0, 0), Square(1, 0), Square(1, 1)],
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
    pub fn len(&self) -> usize {
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

    pub fn to_squares(&self) -> [Square; 4] {
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

    pub fn next_state(&mut self) {
        self.state = (self.state + 1) % self.shape.len();
    }

    pub fn prev_state(&mut self) {
        self.state = (self.state + self.shape.len() - 1) % self.shape.len();
    }
}
