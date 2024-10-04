use std::fmt::Display;

use num_traits::FromPrimitive;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, FromPrimitive)]
pub enum Gravity {
    #[default]
    Level,
    Locked,
}

impl Gravity {
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

    pub fn to_string_abbr(&self) -> String {
        match self {
            Gravity::Level => "LVL",
            Gravity::Locked => "LCK",
        }
        .into()
    }
}

impl Display for Gravity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gravity::Level => f.write_str("LEVEL"),
            Gravity::Locked => f.write_str("LOCKED"),
        }
    }
}
