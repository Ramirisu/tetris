use bevy::prelude::*;
use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, FromRepr};

#[derive(Default, Clone, Copy, FromRepr, EnumIter, EnumCount, Resource, tetris_macros::EnumAdvance)]
pub enum ControllerMapping {
    #[default]
    MappingA,
    MappingB,
}

