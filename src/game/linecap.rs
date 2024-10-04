use std::fmt::Display;

use num_traits::FromPrimitive;

#[derive(Default, Clone, Copy, PartialEq, Eq, FromPrimitive)]
pub enum Linecap {
    #[default]
    Off,
    SuperKillScreen,
}

impl Linecap {
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
            Linecap::Off => "OFF",
            Linecap::SuperKillScreen => "SKS",
        }
        .into()
    }
}

impl Display for Linecap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Linecap::Off => f.write_str("OFF"),
            Linecap::SuperKillScreen => f.write_str("SUPER KILL SCREEN"),
        }
    }
}
