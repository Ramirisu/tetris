use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, FromRepr};

use crate::utility::enum_advance;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, FromRepr, EnumIter, EnumCount)]
pub enum TetrisFlash {
    #[default]
    On,
    Off,
}

enum_advance::enum_advance_derive!(TetrisFlash);
