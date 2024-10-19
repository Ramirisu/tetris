use std::fmt::Display;

use crate::enum_iter;

#[derive(Default, Clone, Copy, PartialEq, Eq, FromPrimitive)]
pub enum Seeding {
    #[default]
    System,
    Custom,
}

enum_iter::enum_iter_derive!(Seeding);

impl Seeding {
    pub fn to_string_abbr(&self) -> String {
        match self {
            Seeding::System => "SYS",
            Seeding::Custom => "CUS",
        }
        .into()
    }
}

impl Display for Seeding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Seeding::System => f.write_str("SYSTEM"),
            Seeding::Custom => f.write_str("CUSTOM"),
        }
    }
}
