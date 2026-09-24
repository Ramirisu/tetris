use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, FromRepr};

#[derive(Default, Clone, Copy, PartialEq, Eq, FromRepr, EnumIter, EnumCount, tetris_macros::EnumAdvance)]
pub enum Seeding {
    #[default]
    System,
    Custom,
}

