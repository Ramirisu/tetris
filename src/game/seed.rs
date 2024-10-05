use std::{collections::VecDeque, fmt::Display};

use num_traits::FromPrimitive;
use rand::Rng;

use super::piece::Piece;

#[derive(Default, Clone, Copy, PartialEq, Eq, FromPrimitive)]
pub enum Seed {
    #[default]
    System,
}

impl Seed {
    pub fn enum_has_prev(&self) -> bool {
        <Self as FromPrimitive>::from_i64(*self as i64 - 1).is_some()
    }

    pub fn enum_has_next(&self) -> bool {
        <Self as FromPrimitive>::from_i64(*self as i64 + 1).is_some()
    }

    pub fn enum_prev(&mut self) -> bool {
        match FromPrimitive::from_i64(*self as i64 - 1) {
            Some(n) => {
                *self = n;
                true
            }
            None => false,
        }
    }

    pub fn enum_next(&mut self) -> bool {
        match FromPrimitive::from_i64(*self as i64 + 1) {
            Some(n) => {
                *self = n;
                true
            }
            None => false,
        }
    }

    pub fn to_string_abbr(&self) -> String {
        match self {
            Seed::System => "SYS",
        }
        .into()
    }

    pub fn gen(&self) -> Piece {
        match self {
            Seed::System => rand::thread_rng()
                .gen_range(0..(Piece::variant_len() - 1))
                .into(),
        }
    }

    pub fn gen_1h2r(&self, history: &VecDeque<Piece>) -> Piece {
        match self {
            Seed::System => match history.back() {
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

impl Display for Seed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Seed::System => f.write_str("SYSTEM"),
        }
    }
}
