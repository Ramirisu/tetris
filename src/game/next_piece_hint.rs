use bevy::prelude::*;
use num_traits::FromPrimitive;

#[derive(Default, Clone, Copy, PartialEq, Eq, FromPrimitive)]
pub enum NextPieceHint {
    Off,
    #[default]
    One,
}

impl NextPieceHint {
    pub fn enum_prev(&mut self) -> Option<Self> {
        FromPrimitive::from_i8(*self as i8 - 1).map(|n| std::mem::replace(self, n))
    }

    pub fn enum_next(&mut self) -> Option<Self> {
        FromPrimitive::from_i8(*self as i8 + 1).map(|n| std::mem::replace(self, n))
    }
}

impl Into<Visibility> for NextPieceHint {
    fn into(self) -> Visibility {
        match self {
            NextPieceHint::Off => Visibility::Hidden,
            NextPieceHint::One => Visibility::Inherited,
        }
    }
}
