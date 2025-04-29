use std::fmt::Display;

use bevy::prelude::*;
use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, FromRepr};

use crate::enum_advance;

#[derive(Default, Clone, Copy, FromRepr, EnumIter, EnumCount, Resource)]
pub enum ShowFPS {
    Off,
    #[default]
    Auto,
    On,
}

enum_advance::enum_advance_derive!(ShowFPS);

impl ShowFPS {
    pub fn is_enabled(&self) -> bool {
        match self {
            ShowFPS::Off => false,
            ShowFPS::Auto => cfg!(debug_assertions),
            ShowFPS::On => true,
        }
    }
}

impl Display for ShowFPS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShowFPS::Off => f.write_str("OFF"),
            ShowFPS::Auto => f.write_str("AUTO"),
            ShowFPS::On => f.write_str("ON"),
        }
    }
}
