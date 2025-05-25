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
    pub fn count(&self) -> usize {
        match self {
            NextPieceHint::Off => 0,
            NextPieceHint::Classic => 1,
            NextPieceHint::Modern => 5,
        }
    }

    pub fn as_visibility(&self, index: usize) -> Visibility {
        if index >= self.count() {
            Visibility::Hidden
        } else {
            Visibility::Inherited
        }
    }
}
