use std::fmt::Display;

use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, FromRepr};

use crate::enum_advance;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, FromRepr, EnumIter, EnumCount)]
pub enum Gravity {
    #[default]
    Level,
    Locked,
}

enum_advance::enum_advance_derive!(Gravity);

impl Gravity {
    pub fn to_str_abbr(&self) -> String {
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
