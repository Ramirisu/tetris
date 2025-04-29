use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, FromRepr};

use crate::{enum_advance, enum_advance_cycle};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Square(pub i32, pub i32);

impl Square {
    pub fn to_coordinate(&self, x: i32, y: i32) -> (i32, i32) {
        (self.0 + x, self.1 + y)
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, FromRepr, EnumIter, EnumCount)]
pub enum PieceT {
    #[default]
    T0,
    T1,
    T2,
    T3,
}

enum_advance::enum_advance_derive!(PieceT);
enum_advance_cycle::enum_advance_cycle_derive!(PieceT);

impl PieceT {
    pub fn to_squares(&self) -> [Square; 4] {
        match self {
            PieceT::T0 => [Square(0, -1), Square(-1, 0), Square(0, 0), Square(1, 0)],
            PieceT::T1 => [Square(0, -1), Square(-1, 0), Square(0, 0), Square(0, 1)],
            PieceT::T2 => [Square(-1, 0), Square(0, 0), Square(1, 0), Square(0, 1)],
            PieceT::T3 => [Square(0, -1), Square(0, 0), Square(1, 0), Square(0, 1)],
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, FromRepr, EnumIter, EnumCount)]
pub enum PieceJ {
    #[default]
    J0,
    J1,
    J2,
    J3,
}

enum_advance::enum_advance_derive!(PieceJ);
enum_advance_cycle::enum_advance_cycle_derive!(PieceJ);

impl PieceJ {
    pub fn to_squares(&self) -> [Square; 4] {
        match self {
            PieceJ::J0 => [Square(-1, 0), Square(0, 0), Square(1, 0), Square(1, -1)],
            PieceJ::J1 => [Square(-1, -1), Square(0, -1), Square(0, 0), Square(0, 1)],
            PieceJ::J2 => [Square(-1, 0), Square(0, 0), Square(1, 0), Square(-1, 1)],
            PieceJ::J3 => [Square(0, -1), Square(0, 0), Square(0, 1), Square(1, 1)],
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, FromRepr, EnumIter, EnumCount)]
pub enum PieceZ {
    #[default]
    Z0,
    Z1,
}

enum_advance::enum_advance_derive!(PieceZ);
enum_advance_cycle::enum_advance_cycle_derive!(PieceZ);

impl PieceZ {
    pub fn to_squares(&self) -> [Square; 4] {
        match self {
            PieceZ::Z0 => [Square(0, -1), Square(1, -1), Square(-1, 0), Square(0, 0)],
            PieceZ::Z1 => [Square(0, -1), Square(0, 0), Square(1, 0), Square(1, 1)],
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, FromRepr, EnumIter, EnumCount)]
pub enum PieceO {
    #[default]
    O0,
}

enum_advance::enum_advance_derive!(PieceO);
enum_advance_cycle::enum_advance_cycle_derive!(PieceO);

impl PieceO {
    pub fn to_squares(&self) -> [Square; 4] {
        [Square(-1, -1), Square(0, -1), Square(-1, 0), Square(0, 0)]
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, FromRepr, EnumIter, EnumCount)]
pub enum PieceS {
    #[default]
    S0,
    S1,
}

enum_advance::enum_advance_derive!(PieceS);
enum_advance_cycle::enum_advance_cycle_derive!(PieceS);

impl PieceS {
    pub fn to_squares(&self) -> [Square; 4] {
        match self {
            PieceS::S0 => [Square(-1, -1), Square(0, -1), Square(0, 0), Square(1, 0)],
            PieceS::S1 => [Square(1, -1), Square(0, 0), Square(1, 0), Square(0, 1)],
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, FromRepr, EnumIter, EnumCount)]
pub enum PieceL {
    #[default]
    L0,
    L1,
    L2,
    L3,
}

enum_advance::enum_advance_derive!(PieceL);
enum_advance_cycle::enum_advance_cycle_derive!(PieceL);

impl PieceL {
    pub fn to_squares(&self) -> [Square; 4] {
        match self {
            PieceL::L0 => [Square(-1, -1), Square(-1, 0), Square(0, 0), Square(1, 0)],
            PieceL::L1 => [Square(0, -1), Square(0, 0), Square(-1, 1), Square(0, 1)],
            PieceL::L2 => [Square(-1, 0), Square(0, 0), Square(1, 0), Square(1, 1)],
            PieceL::L3 => [Square(0, -1), Square(1, -1), Square(0, 0), Square(0, 1)],
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, FromRepr, EnumIter, EnumCount)]
pub enum PieceI {
    #[default]
    I0,
    I1,
}

enum_advance::enum_advance_derive!(PieceI);
enum_advance_cycle::enum_advance_cycle_derive!(PieceI);

impl PieceI {
    pub fn to_squares(&self) -> [Square; 4] {
        match self {
            PieceI::I0 => [Square(-2, 0), Square(-1, 0), Square(0, 0), Square(1, 0)],
            PieceI::I1 => [Square(0, -1), Square(0, 0), Square(0, 1), Square(0, 2)],
        }
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

    pub fn to_squares(&self) -> [Square; 4] {
        match self {
            Piece::T(piece) => piece.to_squares(),
            Piece::J(piece) => piece.to_squares(),
            Piece::Z(piece) => piece.to_squares(),
            Piece::O(piece) => piece.to_squares(),
            Piece::S(piece) => piece.to_squares(),
            Piece::L(piece) => piece.to_squares(),
            Piece::I(piece) => piece.to_squares(),
            Piece::X => panic!("Piece::X is a placeholder"),
        }
    }

    pub fn rotate_clockwise(&mut self) {
        *self = match self {
            Piece::T(piece) => Piece::T(piece.enum_next_cycle()),
            Piece::J(piece) => Piece::J(piece.enum_next_cycle()),
            Piece::Z(piece) => Piece::Z(piece.enum_next_cycle()),
            Piece::O(piece) => Piece::O(piece.enum_next_cycle()),
            Piece::S(piece) => Piece::S(piece.enum_next_cycle()),
            Piece::L(piece) => Piece::L(piece.enum_next_cycle()),
            Piece::I(piece) => Piece::I(piece.enum_next_cycle()),
            Piece::X => Piece::X,
        }
    }

    pub fn rotate_counterclockwise(&mut self) {
        *self = match self {
            Piece::T(piece) => Piece::T(piece.enum_prev_cycle()),
            Piece::J(piece) => Piece::J(piece.enum_prev_cycle()),
            Piece::Z(piece) => Piece::Z(piece.enum_prev_cycle()),
            Piece::O(piece) => Piece::O(piece.enum_prev_cycle()),
            Piece::S(piece) => Piece::S(piece.enum_prev_cycle()),
            Piece::L(piece) => Piece::L(piece.enum_prev_cycle()),
            Piece::I(piece) => Piece::I(piece.enum_prev_cycle()),
            Piece::X => Piece::X,
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
