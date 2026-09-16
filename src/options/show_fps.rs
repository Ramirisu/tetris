use bevy::prelude::*;
use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, FromRepr};

use crate::utility::enum_advance;

#[derive(Default, Clone, Copy, FromRepr, EnumIter, EnumCount, Resource)]
pub enum ShowFPS {
    #[default]
    Off,
    On,
}

enum_advance::enum_advance_derive!(ShowFPS);

impl ShowFPS {
    pub fn is_enabled(&self) -> bool {
        match self {
            ShowFPS::Off => false,
            ShowFPS::On => true,
        }
    }
}
