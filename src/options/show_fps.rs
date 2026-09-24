use bevy::prelude::*;
use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, FromRepr};

#[derive(Default, Clone, Copy, FromRepr, EnumIter, EnumCount, Resource, tetris_macros::EnumAdvance)]
pub enum ShowFPS {
    #[default]
    Off,
    On,
}


impl ShowFPS {
    pub fn is_enabled(&self) -> bool {
        match self {
            ShowFPS::Off => false,
            ShowFPS::On => true,
        }
    }
}
