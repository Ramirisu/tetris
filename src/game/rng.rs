use std::{collections::VecDeque, fmt::Display};

use rand::Rng;

use crate::enum_iter;

use super::piece::Piece;

#[derive(Default, Clone, Copy, PartialEq, Eq, FromPrimitive)]
pub enum RandomNumberGenerator {
    #[default]
    System,
}

enum_iter::enum_iter_derive!(RandomNumberGenerator);

impl RandomNumberGenerator {
    pub fn to_string_abbr(&self) -> String {
        match self {
            RandomNumberGenerator::System => "SYS",
        }
        .into()
    }

    pub fn gen(&self) -> Piece {
        match self {
            RandomNumberGenerator::System => rand::thread_rng()
                .gen_range(0..(Piece::variant_len() - 1))
                .into(),
        }
    }

    pub fn gen_1h2r(&self, history: &VecDeque<Piece>) -> Piece {
        match self {
            RandomNumberGenerator::System => match history.back() {
                Some(piece) => {
                    let index = rand::thread_rng().gen_range(0..Piece::variant_len());
                    if index + 1 != Piece::variant_len() && index != piece.variant_index() {
                        index.into()
                    } else {
                        self.gen()
                    }
                }
                None => self.gen(),
            },
        }
    }
}

impl Display for RandomNumberGenerator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RandomNumberGenerator::System => f.write_str("SYSTEM"),
        }
    }
}
