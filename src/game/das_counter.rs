use std::fmt::Display;

use bevy::prelude::*;
use num_traits::FromPrimitive;

#[derive(Default, Clone, Copy, PartialEq, Eq, FromPrimitive)]
pub enum DASCounter {
    Off,
    #[default]
    Default,
    Full,
}

impl DASCounter {
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

    pub fn get_counter_visibility(&self) -> Visibility {
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