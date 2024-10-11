use std::fmt::Display;

use bevy::prelude::*;

use crate::enum_iter;

#[derive(Default, Clone, Copy, PartialEq, Eq, FromPrimitive)]
pub enum DASCounter {
    Off,
    #[default]
    Default,
    Full,
}

enum_iter::enum_iter_derive!(DASCounter);

impl Into<Visibility> for DASCounter {
    fn into(self) -> Visibility {
        match self {
            DASCounter::Off => Visibility::Hidden,
            DASCounter::Default => Visibility::Inherited,
            DASCounter::Full => Visibility::Inherited,
        }
    }
}

impl Display for DASCounter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DASCounter::Off => f.write_str("OFF"),
            DASCounter::Default => f.write_str("DEFAULT"),
            DASCounter::Full => f.write_str("FULL"),
        }
    }
}
