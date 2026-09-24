use std::collections::VecDeque;

use rand::{RngExt, seq::SliceRandom};
use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, FromRepr};

use super::piece::Piece;

pub type PieceHistory = VecDeque<Piece>;

#[derive(Default, Clone, Copy, PartialEq, Eq, FromRepr, EnumIter, EnumCount, tetris_macros::EnumAdvance)]
pub enum Random {
    Uniform,
    #[default]
    Classic,
    Modern,
}


impl Random {
    pub fn gen_pieces<R: rand::Rng>(&self, rng: &mut R, history: &PieceHistory) -> Vec<Piece> {
        match self {
            Random::Uniform => vec![Self::gen_piece_uniform(rng)],
            Random::Classic => vec![Self::gen_piece_1h2r(rng, history)],
            Random::Modern => Self::gen_bag(rng),
        }
    }

    fn gen_piece_uniform<R: rand::Rng>(rng: &mut R) -> Piece {
        let index = rng.random_range(0..(Piece::variant_len() - 1));
        Piece::from(index)
    }

    // NES Tetris 1H2R:
    // There are 7 piece types, numbered 0-6. Generate a random value from 0-7.
    // If the value is invalid (7) or matches the previous piece, reroll from 0-6.
    fn gen_piece_1h2r<R: rand::Rng>(rng: &mut R, history: &PieceHistory) -> Piece {
        let Some(last_piece) = history.back() else {
            return Self::gen_piece_uniform(rng);
        };

        let index = rng.random_range(0..Piece::variant_len());
        if index + 1 != Piece::variant_len() && index != last_piece.variant_index() {
            index.into()
        } else {
            Self::gen_piece_uniform(rng)
        }
    }

    fn gen_bag<R: rand::Rng>(rng: &mut R) -> Vec<Piece> {
        let mut pieces = Piece::iter()
            .filter(|piece| !piece.is_placeholder())
            .copied()
            .collect::<Vec<_>>();
        pieces.shuffle(rng);
        pieces
    }
}
