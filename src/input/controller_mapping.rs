use std::fmt::Display;

use bevy::prelude::*;

use crate::enum_iter;

#[derive(Default, Clone, Copy, FromPrimitive, Resource)]
pub enum ControllerMapping {
    #[default]
    MappingA,
    MappingB,
}

enum_iter::enum_iter_derive!(ControllerMapping);

impl Display for ControllerMapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ControllerMapping::MappingA => f.write_str("MAPPING A"),
            ControllerMapping::MappingB => f.write_str("MAPPING B"),
        }
    }
}
