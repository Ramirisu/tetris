use num_traits::FromPrimitive;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Square(pub i32, pub i32);

impl Square {
    pub fn to_coordinate(&self, x: i32, y: i32) -> (i32, i32) {
        (self.0 + x, self.1 + y)
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, FromPrimitive)]
pub enum PieceT {
    #[default]
    T0,
    T1,
    T2,
    T3,
}

impl PieceT {
    pub fn get_squares(&self) -> [Square; 4] {
        match self {
            PieceT::T0 => [Square(0, -1), Square(-1, 0), Square(0, 0), Square(1, 0)],
            PieceT::T1 => [Square(0, -1), Square(-1, 0), Square(0, 0), Square(0, 1)],
            PieceT::T2 => [Square(-1, 0), Square(0, 0), Square(1, 0), Square(0, 1)],
            PieceT::T3 => [Square(0, -1), Square(0, 0), Square(1, 0), Square(0, 1)],
        }
    }

    pub fn rotate_clockwise(&mut self) {
        match FromPrimitive::from_i8(*self as i8 + 1) {
            Some(p) => *self = p,
            None => *self = PieceT::T0,
        };
    }

    pub fn rotate_counterclockwise(&mut self) {
        match FromPrimitive::from_i8(*self as i8 - 1) {
            Some(p) => *self = p,
            None => *self = PieceT::T3,
        };
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, FromPrimitive)]
pub enum PieceJ {
    #[default]
    J0,
    J1,
    J2,
    J3,
}

impl PieceJ {
    pub fn get_squares(&self) -> [Square; 4] {
        match self {
            PieceJ::J0 => [Square(-1, 0), Square(0, 0), Square(1, 0), Square(1, -1)],
            PieceJ::J1 => [Square(-1, -1), Square(0, -1), Square(0, 0), Square(0, 1)],
            PieceJ::J2 => [Square(-1, 0), Square(0, 0), Square(1, 0), Square(-1, 1)],
            PieceJ::J3 => [Square(0, -1), Square(0, 0), Square(0, 1), Square(1, 1)],
        }
    }

    pub fn rotate_clockwise(&mut self) {
        match FromPrimitive::from_i8(*self as i8 + 1) {
            Some(p) => *self = p,
            None => *self = PieceJ::J0,
        };
    }

    pub fn rotate_counterclockwise(&mut self) {
        match FromPrimitive::from_i8(*self as i8 - 1) {
            Some(p) => *self = p,
            None => *self = PieceJ::J3,
        };
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, FromPrimitive)]
pub enum PieceZ {
    #[default]
    Z0,
    Z1,
}

impl PieceZ {
    pub fn get_squares(&self) -> [Square; 4] {
        match self {
            PieceZ::Z0 => [Square(0, -1), Square(1, -1), Square(-1, 0), Square(0, 0)],
            PieceZ::Z1 => [Square(0, -1), Square(0, 0), Square(1, 0), Square(1, 1)],
        }
    }

    pub fn rotate_clockwise(&mut self) {
        match FromPrimitive::from_i8(*self as i8 + 1) {
            Some(p) => *self = p,
            None => *self = PieceZ::Z0,
        };
    }

    pub fn rotate_counterclockwise(&mut self) {
        match FromPrimitive::from_i8(*self as i8 - 1) {
            Some(p) => *self = p,
            None => *self = PieceZ::Z1,
        };
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum PieceO {
    #[default]
    O0,
}

impl PieceO {
    pub fn get_squares(&self) -> [Square; 4] {
        [Square(-1, -1), Square(0, -1), Square(-1, 0), Square(0, 0)]
    }

    pub fn rotate_clockwise(&mut self) {}

    pub fn rotate_counterclockwise(&mut self) {}
}

#[derive(Default, Clone, Copy, PartialEq, Eq, FromPrimitive)]
pub enum PieceS {
    #[default]
    S0,
    S1,
}

impl PieceS {
    pub fn get_squares(&self) -> [Square; 4] {
        match self {
            PieceS::S0 => [Square(-1, -1), Square(0, -1), Square(0, 0), Square(1, 0)],
            PieceS::S1 => [Square(1, -1), Square(0, 0), Square(1, 0), Square(0, 1)],
        }
    }

    pub fn rotate_clockwise(&mut self) {
        match FromPrimitive::from_i8(*self as i8 + 1) {
            Some(p) => *self = p,
            None => *self = PieceS::S0,
        };
    }

    pub fn rotate_counterclockwise(&mut self) {
        match FromPrimitive::from_i8(*self as i8 - 1) {
            Some(p) => *self = p,
            None => *self = PieceS::S1,
        };
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, FromPrimitive)]
pub enum PieceL {
    #[default]
    L0,
    L1,
    L2,
    L3,
}

impl PieceL {
    pub fn get_squares(&self) -> [Square; 4] {
        match self {
            PieceL::L0 => [Square(-1, -1), Square(-1, 0), Square(0, 0), Square(1, 0)],
            PieceL::L1 => [Square(0, -1), Square(0, 0), Square(-1, 1), Square(0, 1)],
            PieceL::L2 => [Square(-1, 0), Square(0, 0), Square(1, 0), Square(1, 1)],
            PieceL::L3 => [Square(0, -1), Square(1, -1), Square(0, 0), Square(0, 1)],
        }
    }

    pub fn rotate_clockwise(&mut self) {
        match FromPrimitive::from_i8(*self as i8 + 1) {
            Some(p) => *self = p,
            None => *self = PieceL::L0,
        };
    }

    pub fn rotate_counterclockwise(&mut self) {
        match FromPrimitive::from_i8(*self as i8 - 1) {
            Some(p) => *self = p,
            None => *self = PieceL::L3,
        };
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, FromPrimitive)]
pub enum PieceI {
    #[default]
    I0,
    I1,
}

impl PieceI {
    pub fn get_squares(&self) -> [Square; 4] {
        match self {
            PieceI::I0 => [Square(-2, 0), Square(-1, 0), Square(0, 0), Square(1, 0)],
            PieceI::I1 => [Square(0, -1), Square(0, 0), Square(0, 1), Square(0, 2)],
        }
    }

    pub fn rotate_clockwise(&mut self) {
        match FromPrimitive::from_i8(*self as i8 + 1) {
            Some(p) => *self = p,
            None => *self = PieceI::I0,
        };
    }

    pub fn rotate_counterclockwise(&mut self) {
        match FromPrimitive::from_i8(*self as i8 - 1) {
            Some(p) => *self = p,
            None => *self = PieceI::I1,
        };
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    T(PieceT),
    J(PieceJ),
    Z(PieceZ),
    O(PieceO),
    S(PieceS),
    L(PieceL),
    I(PieceI),
    #[default]
    X, // placeholder for the type without a shape
}

impl Piece {
    pub const fn variant_len() -> usize {
        8
    }

    pub fn new_t() -> Self {
        Piece::T(PieceT::default())
    }

    pub fn new_j() -> Self {
        Piece::J(PieceJ::default())
    }

    pub fn new_z() -> Self {
        Piece::Z(PieceZ::default())
    }

    pub fn new_o() -> Self {
        Piece::O(PieceO::default())
    }

    pub fn new_s() -> Self {
        Piece::S(PieceS::default())
    }

    pub fn new_l() -> Self {
        Piece::L(PieceL::default())
    }

    pub fn new_i() -> Self {
        Piece::I(PieceI::default())
    }

    pub fn is_placeholder(&self) -> bool {
        *self == Piece::X
    }

    pub fn variant_index(&self) -> usize {
        match self {
            Piece::T(_) => 0,
            Piece::J(_) => 1,
            Piece::Z(_) => 2,
            Piece::O(_) => 3,
            Piece::S(_) => 4,
            Piece::L(_) => 5,
            Piece::I(_) => 6,
            Piece::X => 7,
        }
    }

    pub fn get_squares(&self) -> [Square; 4] {
        match self {
            Piece::T(piece) => piece.get_squares(),
            Piece::J(piece) => piece.get_squares(),
            Piece::Z(piece) => piece.get_squares(),
            Piece::O(piece) => piece.get_squares(),
            Piece::S(piece) => piece.get_squares(),
            Piece::L(piece) => piece.get_squares(),
            Piece::I(piece) => piece.get_squares(),
            Piece::X => panic!("Piece::X is a placeholder"),
        }
    }

    pub fn get_center_offset(&self) -> (f32, f32) {
        let mut maxx = 0;
        let mut minx = 0;
        let mut maxy = 0;
        let mut miny = 0;

        self.get_squares().iter().for_each(|sqr| {
            maxx = maxx.max(sqr.0);
            minx = minx.min(sqr.0);
            maxy = maxy.max(sqr.1);
            miny = miny.min(sqr.1);
        });

        ((maxx + minx) as f32 / -2.0, (maxy + miny) as f32 / -2.0)
    }

    pub fn rotate_clockwise(&mut self) {
        match self {
            Piece::T(piece) => piece.rotate_clockwise(),
            Piece::J(piece) => piece.rotate_clockwise(),
            Piece::Z(piece) => piece.rotate_clockwise(),
            Piece::O(piece) => piece.rotate_clockwise(),
            Piece::S(piece) => piece.rotate_clockwise(),
            Piece::L(piece) => piece.rotate_clockwise(),
            Piece::I(piece) => piece.rotate_clockwise(),
            Piece::X => (),
        }
    }

    pub fn rotate_counterclockwise(&mut self) {
        match self {
            Piece::T(piece) => piece.rotate_counterclockwise(),
            Piece::J(piece) => piece.rotate_counterclockwise(),
            Piece::Z(piece) => piece.rotate_counterclockwise(),
            Piece::O(piece) => piece.rotate_counterclockwise(),
            Piece::S(piece) => piece.rotate_counterclockwise(),
            Piece::L(piece) => piece.rotate_counterclockwise(),
            Piece::I(piece) => piece.rotate_counterclockwise(),
            Piece::X => (),
        }
    }

    pub fn iter() -> std::slice::Iter<'static, Piece> {
        const PIECES: [Piece; 8] = [
            Piece::T(PieceT::T0),
            Piece::J(PieceJ::J0),
            Piece::Z(PieceZ::Z0),
            Piece::O(PieceO::O0),
            Piece::S(PieceS::S0),
            Piece::L(PieceL::L0),
            Piece::I(PieceI::I0),
            Piece::X,
        ];
        PIECES.iter()
    }
}

impl From<usize> for Piece {
    fn from(value: usize) -> Self {
        match value {
            0 => Piece::new_t(),
            1 => Piece::new_j(),
            2 => Piece::new_z(),
            3 => Piece::new_o(),
            4 => Piece::new_s(),
            5 => Piece::new_l(),
            6 => Piece::new_i(),
            7 => Piece::X,
            _ => panic!("value exceeds range"),
        }
    }
}
