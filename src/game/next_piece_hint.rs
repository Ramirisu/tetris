use std::fmt::Display;

use bevy::prelude::*;
use num_traits::FromPrimitive;

#[derive(Default, Clone, Copy, PartialEq, Eq, FromPrimitive)]
pub enum NextPieceHint {
    Off,
    #[default]
    Classic,
    Modern,
}

impl NextPieceHint {
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

    pub fn get_visibility(&self, index: usize) -> Visibility {
        let visible = match self {
            NextPieceHint::Off => 0,
            NextPieceHint::Classic => 1,
            NextPieceHint::Modern => 5,
        };
        if index >= visible {
            Visibility::Hidden
        } else {
            Visibility::Inherited
        }
    }
}

impl Display for NextPieceHint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NextPieceHint::Off => f.write_str("OFF"),
            NextPieceHint::Classic => f.write_str("CLASSIC"),
            NextPieceHint::Modern => f.write_str("MODERN"),
        }
    }
}
