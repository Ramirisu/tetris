use std::fmt::Display;

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

impl Invisible {
    pub fn to_string_abbr(&self) -> String {
        match self {
            Invisible::Off => "OFF",
            Invisible::On => "ON",
        }
        .into()
    }
}

impl Into<Visibility> for Invisible {
    fn into(self) -> Visibility {
        match self {
            Invisible::Off => Visibility::Inherited,
            Invisible::On => Visibility::Hidden,
        }
    }
}

impl Display for Invisible {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Invisible::Off => f.write_str("OFF"),
            Invisible::On => f.write_str("ON"),
        }
    }
}
