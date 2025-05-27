use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, FromRepr};

use crate::utility::enum_advance;

use super::level::Level;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, FromRepr, EnumIter, EnumCount)]
pub enum Transition {
    #[default]
    Classic,
    Fixed,
    Every10Lines,
    Every4Lines,
}

enum_advance::enum_advance_derive!(Transition);

impl Transition {
    pub fn transform(&self, start_level: Level, lines: usize) -> Level {
        match self {
            Transition::Classic => Self::transform_classic(start_level, lines),
            Transition::Fixed => Self::transform_fixed(start_level, lines),
            Transition::Every10Lines => Self::transform_every_n_lines(start_level, lines, 10),
            Transition::Every4Lines => Self::transform_every_n_lines(start_level, lines, 4),
        }
    }

    fn transform_classic(start_level: Level, lines: usize) -> Level {
        if start_level < 10 {
            return Level(start_level.0.max(lines / 10));
        } else if start_level <= 15 {
            if lines >= 100 {
                return Level(start_level.0 + lines / 10 - 9);
            }
        } else if start_level < 25 {
            if lines >= start_level.0 * 10 - 50 {
                return Level(lines / 10 + 6);
            }
        } else {
            if lines >= 200 {
                return Level(start_level.0 + lines / 10 - 19);
            }
        }

        start_level
    }

    fn transform_fixed(start_level: Level, lines: usize) -> Level {
        Level(start_level.0.max(lines / 10))
    }

    fn transform_every_n_lines(start_level: Level, lines: usize, every: usize) -> Level {
        Level(start_level.0 + lines / every)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classic() {
        let transition = Transition::Classic;

        assert_eq!(transition.transform(Level(0), 0), Level(0));
        assert_eq!(transition.transform(Level(0), 10), Level(1));
        assert_eq!(transition.transform(Level(0), 20), Level(2));
        assert_eq!(transition.transform(Level(0), 30), Level(3));
        assert_eq!(transition.transform(Level(0), 100), Level(10));
        assert_eq!(transition.transform(Level(0), 130), Level(13));
        assert_eq!(transition.transform(Level(0), 190), Level(19));
        assert_eq!(transition.transform(Level(0), 290), Level(29));

        assert_eq!(transition.transform(Level(9), 0), Level(9));
        assert_eq!(transition.transform(Level(9), 10), Level(9));
        assert_eq!(transition.transform(Level(9), 20), Level(9));
        assert_eq!(transition.transform(Level(9), 30), Level(9));
        assert_eq!(transition.transform(Level(9), 100), Level(10));
        assert_eq!(transition.transform(Level(9), 130), Level(13));
        assert_eq!(transition.transform(Level(9), 190), Level(19));
        assert_eq!(transition.transform(Level(9), 290), Level(29));

        assert_eq!(transition.transform(Level(10), 0), Level(10));
        assert_eq!(transition.transform(Level(10), 10), Level(10));
        assert_eq!(transition.transform(Level(10), 20), Level(10));
        assert_eq!(transition.transform(Level(10), 30), Level(10));
        assert_eq!(transition.transform(Level(10), 100), Level(11));
        assert_eq!(transition.transform(Level(10), 130), Level(14));
        assert_eq!(transition.transform(Level(10), 180), Level(19));
        assert_eq!(transition.transform(Level(10), 280), Level(29));

        assert_eq!(transition.transform(Level(15), 0), Level(15));
        assert_eq!(transition.transform(Level(15), 10), Level(15));
        assert_eq!(transition.transform(Level(15), 20), Level(15));
        assert_eq!(transition.transform(Level(15), 30), Level(15));
        assert_eq!(transition.transform(Level(15), 100), Level(16));
        assert_eq!(transition.transform(Level(15), 130), Level(19));
        assert_eq!(transition.transform(Level(15), 230), Level(29));

        assert_eq!(transition.transform(Level(16), 0), Level(16));
        assert_eq!(transition.transform(Level(16), 10), Level(16));
        assert_eq!(transition.transform(Level(16), 20), Level(16));
        assert_eq!(transition.transform(Level(16), 30), Level(16));
        assert_eq!(transition.transform(Level(16), 110), Level(17));
        assert_eq!(transition.transform(Level(16), 130), Level(19));
        assert_eq!(transition.transform(Level(16), 230), Level(29));

        assert_eq!(transition.transform(Level(18), 0), Level(18));
        assert_eq!(transition.transform(Level(18), 10), Level(18));
        assert_eq!(transition.transform(Level(18), 20), Level(18));
        assert_eq!(transition.transform(Level(18), 30), Level(18));
        assert_eq!(transition.transform(Level(18), 120), Level(18));
        assert_eq!(transition.transform(Level(18), 130), Level(19));
        assert_eq!(transition.transform(Level(18), 230), Level(29));

        assert_eq!(transition.transform(Level(19), 0), Level(19));
        assert_eq!(transition.transform(Level(19), 10), Level(19));
        assert_eq!(transition.transform(Level(19), 20), Level(19));
        assert_eq!(transition.transform(Level(19), 30), Level(19));
        assert_eq!(transition.transform(Level(19), 130), Level(19));
        assert_eq!(transition.transform(Level(19), 140), Level(20));
        assert_eq!(transition.transform(Level(19), 230), Level(29));

        assert_eq!(transition.transform(Level(26), 0), Level(26));
        assert_eq!(transition.transform(Level(26), 10), Level(26));
        assert_eq!(transition.transform(Level(26), 20), Level(26));
        assert_eq!(transition.transform(Level(26), 30), Level(26));
        assert_eq!(transition.transform(Level(26), 190), Level(26));
        assert_eq!(transition.transform(Level(26), 200), Level(27));
        assert_eq!(transition.transform(Level(26), 220), Level(29));
    }

    #[test]
    fn test_every_n_lines() {
        let transition = Transition::Every10Lines;

        assert_eq!(transition.transform(Level(0), 0), Level(0));
        assert_eq!(transition.transform(Level(0), 10), Level(1));
        assert_eq!(transition.transform(Level(0), 20), Level(2));
        assert_eq!(transition.transform(Level(0), 30), Level(3));
        assert_eq!(transition.transform(Level(0), 180), Level(18));

        assert_eq!(transition.transform(Level(18), 0), Level(18));
        assert_eq!(transition.transform(Level(18), 10), Level(19));
        assert_eq!(transition.transform(Level(18), 20), Level(20));
        assert_eq!(transition.transform(Level(18), 30), Level(21));
        assert_eq!(transition.transform(Level(18), 180), Level(36));
    }
}
