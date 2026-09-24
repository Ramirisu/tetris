use bevy::prelude::Visibility;
use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, FromRepr};

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    FromRepr,
    EnumIter,
    EnumCount,
    tetris_macros::EnumAdvance,
)]
pub enum Invisible {
    #[default]
    Off,
    On,
}

impl Into<Visibility> for Invisible {
    fn into(self) -> Visibility {
        match self {
            Invisible::Off => Visibility::Inherited,
            Invisible::On => Visibility::Hidden,
        }
    }
}
