use bevy::prelude::*;
use num_traits::FromPrimitive;

#[derive(Default, Clone, Copy, PartialEq, Eq, FromPrimitive)]
pub enum NextPieceHint {
    Off,
    #[default]
    Classic, // 1
    Modern, // 5
}

impl NextPieceHint {
    pub fn enum_prev(&mut self) -> Option<Self> {
        FromPrimitive::from_i8(*self as i8 - 1).map(|n| std::mem::replace(self, n))
    }

    pub fn enum_next(&mut self) -> Option<Self> {
        FromPrimitive::from_i8(*self as i8 + 1).map(|n| std::mem::replace(self, n))
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
