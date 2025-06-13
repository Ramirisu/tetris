use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, FromRepr};

use crate::utility::enum_advance;

#[derive(Default, Clone, Copy, PartialEq, Eq, FromRepr, EnumIter, EnumCount)]
pub enum Linecap {
    #[default]
    Off,
    KillScreenX2,
    Halt,
}

enum_advance::enum_advance_derive!(Linecap);
