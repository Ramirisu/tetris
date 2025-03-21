use std::fmt::Display;

use bevy::prelude::*;
use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, FromRepr};

use crate::enum_advance;

#[derive(Default, Clone, Copy, FromRepr, EnumIter, EnumCount, Resource)]
pub enum ControllerMapping {
    #[default]
    MappingA,
    MappingB,
}

enum_advance::enum_advance_derive!(ControllerMapping);

impl Display for ControllerMapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ControllerMapping::MappingA => f.write_str("MAPPING A"),
            ControllerMapping::MappingB => f.write_str("MAPPING B"),
        }
    }
}
