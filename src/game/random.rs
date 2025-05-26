use std::collections::VecDeque;

use rand::seq::SliceRandom;
use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, FromRepr};

use crate::enum_advance;

use super::piece::Piece;

pub type PieceHistory = VecDeque<Piece>;

#[derive(Default, Clone, Copy, PartialEq, Eq, FromRepr, EnumIter, EnumCount)]
pub enum Random {
    Uniform,
    #[default]
    Classic,
    Modern,
}

enum_advance::enum_advance_derive!(Random);

impl Random {
    pub fn gen_pieces<R: rand::Rng>(&self, rng: &mut R, history: &PieceHistory) -> Vec<Piece> {
        match self {
            Random::Uniform => vec![Self::gen_piece_uniform(rng)],
            Random::Classic => Self::gen_pieces_1h2r(rng, history),
            Random::Modern => Self::gen_pieces_7bag(rng, history),
        }
    }

    fn gen_piece_uniform<R: rand::Rng>(rng: &mut R) -> Piece {
        rng.random_range(0..(Piece::variant_len() - 1)).into()
    }

    fn gen_pieces_1h2r<R: rand::Rng>(rng: &mut R, history: &PieceHistory) -> Vec<Piece> {
        let piece = match history.back() {
            Some(piece) => {
                let index = rng.random_range(0..Piece::variant_len());
                if index + 1 != Piece::variant_len() && index != piece.variant_index() {
                    index.into()
                } else {
                    Self::gen_piece_uniform(rng)
                }
            }
            None => Self::gen_piece_uniform(rng),
        };

        vec![piece]
    }

    fn gen_pieces_7bag<R: rand::Rng>(rng: &mut R, _history: &PieceHistory) -> Vec<Piece> {
        let mut pieces = Piece::iter()
            .filter(|piece| !piece.is_placeholder())
            .cloned()
            .collect::<Vec<Piece>>();
        pieces.shuffle(rng);
        pieces
    }
}
