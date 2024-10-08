use std::fmt::Display;

use crate::enum_iter;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, FromPrimitive)]
pub enum Gravity {
    #[default]
    Level,
    Locked,
}

enum_iter::enum_iter_derive!(Gravity);

impl Gravity {
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
