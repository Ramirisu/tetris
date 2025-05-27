use bevy::prelude::*;
use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, FromRepr};

use crate::utility::enum_advance;

#[derive(Default, Clone, Copy, FromRepr, EnumIter, EnumCount, Resource)]
pub enum ControllerMapping {
    #[default]
    MappingA,
    MappingB,
}

enum_advance::enum_advance_derive!(ControllerMapping);
