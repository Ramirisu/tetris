use std::fmt::Display;

use bevy::prelude::*;
use num_traits::FromPrimitive;

#[derive(Default, Clone, Copy, FromPrimitive, Resource)]
pub enum ControllerMapping {
    #[default]
    MappingA,
    MappingB,
}

impl ControllerMapping {
    pub fn enum_has_prev(&self) -> bool {
        <Self as FromPrimitive>::from_i64(*self as i64 - 1).is_some()
    }

    pub fn enum_has_next(&self) -> bool {
        <Self as FromPrimitive>::from_i64(*self as i64 + 1).is_some()
    }

    pub fn enum_prev(&mut self) -> bool {
        match FromPrimitive::from_i64(*self as i64 - 1) {
            Some(n) => {
                *self = n;
                true
            }
            None => false,
        }
    }

    pub fn enum_next(&mut self) -> bool {
        match FromPrimitive::from_i64(*self as i64 + 1) {
            Some(n) => {
                *self = n;
                true
            }
            None => false,
        }
    }
}

impl Display for ControllerMapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ControllerMapping::MappingA => f.write_str("MAPPING A"),
            ControllerMapping::MappingB => f.write_str("MAPPING B"),
        }
    }
}
