use num_traits::FromPrimitive;
use rand::Rng;

use super::piece::Piece;

#[derive(Default, Clone, Copy, PartialEq, Eq, FromPrimitive)]
pub enum PieceRandomizer {
    #[default]
    System,
}

impl PieceRandomizer {
    pub fn enum_prev(&mut self) -> Option<Self> {
        FromPrimitive::from_i8(*self as i8 - 1).map(|n| std::mem::replace(self, n))
    }

    pub fn enum_next(&mut self) -> Option<Self> {
        FromPrimitive::from_i8(*self as i8 + 1).map(|n| std::mem::replace(self, n))
    }

    pub fn gen(&self) -> Piece {
        match self {
            PieceRandomizer::System => rand::thread_rng()
                .gen_range(0..(Piece::variant_len() - 1))
                .into(),
        }
    }

    pub fn gen_1h2r(&self, piece: Piece) -> Piece {
        match self {
            PieceRandomizer::System => {
                let index = rand::thread_rng().gen_range(0..Piece::variant_len());
                if index + 1 != Piece::variant_len() && index != piece.variant_index() {
                    index.into()
                } else {
                    self.gen()
                }
            }
        }
    }
}
