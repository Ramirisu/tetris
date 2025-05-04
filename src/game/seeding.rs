
use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, FromRepr};

use crate::enum_advance;

#[derive(Default, Clone, Copy, PartialEq, Eq, FromRepr, EnumIter, EnumCount)]
pub enum Seeding {
    #[default]
    System,
    Custom,
}

enum_advance::enum_advance_derive!(Seeding);

impl Seeding {
    pub fn to_str_abbr(&self) -> String {
        match self {
            Seeding::System => "SYS",
            Seeding::Custom => "CUS",
        }
        .into()
    }
}
