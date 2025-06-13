use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, FromRepr};

use crate::utility::enum_advance;

#[derive(Default, Clone, Copy, PartialEq, Eq, FromRepr, EnumIter, EnumCount)]
pub enum Seeding {
    #[default]
    System,
    Custom,
}

enum_advance::enum_advance_derive!(Seeding);
