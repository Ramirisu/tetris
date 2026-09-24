use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, FromRepr};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, FromRepr, EnumIter, EnumCount, tetris_macros::EnumAdvance)]
pub enum TetrisFlash {
    #[default]
    On,
    Off,
}

