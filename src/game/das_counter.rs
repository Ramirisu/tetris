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
    pub fn enum_prev(&mut self) -> Option<Self> {
        FromPrimitive::from_i8(*self as i8 - 1).map(|n| std::mem::replace(self, n))
    }

    pub fn enum_next(&mut self) -> Option<Self> {
        FromPrimitive::from_i8(*self as i8 + 1).map(|n| std::mem::replace(self, n))
    }

    pub fn get_counter_visibility(&self) -> Visibility {
        match self {
            DASCounter::Off => Visibility::Hidden,
            DASCounter::Default => Visibility::Inherited,
            DASCounter::Full => Visibility::Inherited,
        }
    }
}
