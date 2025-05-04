
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
    pub fn to_str_abbr(&self) -> String {
        match self {
            Linecap::Off => "OFF",
            Linecap::SuperKillScreen => "SKS",
        }
        .into()
    }
}
