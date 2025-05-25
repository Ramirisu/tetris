use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, FromRepr};

use crate::enum_advance;

use super::level::Level;

#[derive(Default, Clone, Copy, PartialEq, Eq, FromRepr, EnumIter, EnumCount)]
pub enum Scoring {
    #[default]
    Classic,
}

enum_advance::enum_advance_derive!(Scoring);

impl Scoring {
    pub fn transform(&self, lines: usize, level: Level) -> usize {
        match *self {
            Scoring::Classic => {
                (level.0 + 1)
                    * match lines {
                        1 => 40,
                        2 => 100,
                        3 => 300,
                        4 => 1200,
                        _ => panic!("can only clear lines between 1-4"),
                    }
            }
        }
    }
}
