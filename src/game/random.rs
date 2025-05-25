use std::collections::VecDeque;

use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, FromRepr};

use crate::enum_advance;

use super::piece::Piece;

pub type PieceHistory = VecDeque<Piece>;

#[derive(Default, Clone, Copy, PartialEq, Eq, FromRepr, EnumIter, EnumCount)]
pub enum Random {
    #[default]
    Classic,
}

enum_advance::enum_advance_derive!(Random);

impl Random {
    pub fn gen_piece<R: rand::Rng>(&self, rng: &mut R, history: &PieceHistory) -> Piece {
        match self {
            Random::Classic => Self::gen_piece_1h2r(rng, history),
        }
    }

    fn gen_piece_uniform<R: rand::Rng>(rng: &mut R) -> Piece {
        rng.random_range(0..(Piece::variant_len() - 1)).into()
    }

    fn gen_piece_1h2r<R: rand::Rng>(rng: &mut R, history: &PieceHistory) -> Piece {
        match history.back() {
            Some(piece) => {
                let index = rng.random_range(0..Piece::variant_len());
                if index + 1 != Piece::variant_len() && index != piece.variant_index() {
                    index.into()
                } else {
                    Self::gen_piece_uniform(rng)
                }
            }
            None => Self::gen_piece_uniform(rng),
        }
    }
}
