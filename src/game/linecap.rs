use std::fmt::Display;

use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, FromRepr};

use crate::enum_advance;

#[derive(Default, Clone, Copy, PartialEq, Eq, FromRepr, EnumIter, EnumCount)]
pub enum Linecap {
    #[default]
    Off,
    SuperKillScreen,
}

enum_advance::enum_advance_derive!(Linecap);

impl Linecap {
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
