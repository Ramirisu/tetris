use bevy::prelude::Visibility;
use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, FromRepr};

use crate::enum_advance;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, FromRepr, EnumIter, EnumCount)]
pub enum Invisible {
    #[default]
    Off,
    On,
}

enum_advance::enum_advance_derive!(Invisible);

impl Into<Visibility> for Invisible {
    fn into(self) -> Visibility {
        match self {
            Invisible::Off => Visibility::Inherited,
            Invisible::On => Visibility::Hidden,
        }
    }
}
