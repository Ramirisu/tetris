use std::fmt::Display;

use bevy::prelude::*;
use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, FromRepr};

use crate::enum_advance;

#[derive(Default, Clone, Copy, PartialEq, Eq, FromRepr, EnumIter, EnumCount)]
pub enum NextPieceHint {
    Off,
    #[default]
    Classic,
    Modern,
}

enum_advance::enum_advance_derive!(NextPieceHint);

impl NextPieceHint {
    pub fn as_visibility(&self, index: usize) -> Visibility {
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
