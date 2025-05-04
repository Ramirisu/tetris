
use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, FromRepr};

use crate::enum_advance;

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
    pub fn to_str_abbr(&self) -> String {
        match self {
            Transition::Classic => "CLS",
            Transition::Fixed => "FIX",
            Transition::Every10Lines => "10L",
            Transition::Every4Lines => "4L",
        }
        .into()
    }

    pub fn transform_level(&self, start_level: usize, lines: usize) -> usize {
        match self {
            Transition::Classic => Self::tf_lv_classic(start_level, lines),
            Transition::Fixed => Self::tf_lv_fixed(start_level, lines),
            Transition::Every10Lines => Self::tf_lv_every_n_lines(start_level, lines, 10),
            Transition::Every4Lines => Self::tf_lv_every_n_lines(start_level, lines, 4),
        }
    }

    fn tf_lv_classic(start_level: usize, lines: usize) -> usize {
        if start_level < 10 {
            return std::cmp::max(start_level, lines / 10);
        } else if start_level <= 15 {
            if lines >= 100 {
                return start_level + lines / 10 - 9;
            }
        } else if start_level < 25 {
            if lines >= start_level * 10 - 50 {
                return lines / 10 + 6;
            }
        } else {
            if lines >= 200 {
                return start_level + lines / 10 - 19;
            }
        }

        start_level
    }

    fn tf_lv_fixed(start_level: usize, lines: usize) -> usize {
        start_level.max(lines / 10)
    }

    fn tf_lv_every_n_lines(start_level: usize, lines: usize, every: usize) -> usize {
        start_level + lines / every
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classic() {
        let transition = Transition::Classic;

        assert_eq!(transition.transform_level(0, 0), 0);
        assert_eq!(transition.transform_level(0, 10), 1);
        assert_eq!(transition.transform_level(0, 20), 2);
        assert_eq!(transition.transform_level(0, 30), 3);
        assert_eq!(transition.transform_level(0, 100), 10);
        assert_eq!(transition.transform_level(0, 130), 13);
        assert_eq!(transition.transform_level(0, 190), 19);
        assert_eq!(transition.transform_level(0, 290), 29);

        assert_eq!(transition.transform_level(9, 0), 9);
        assert_eq!(transition.transform_level(9, 10), 9);
        assert_eq!(transition.transform_level(9, 20), 9);
        assert_eq!(transition.transform_level(9, 30), 9);
        assert_eq!(transition.transform_level(9, 100), 10);
        assert_eq!(transition.transform_level(9, 130), 13);
        assert_eq!(transition.transform_level(9, 190), 19);
        assert_eq!(transition.transform_level(9, 290), 29);

        assert_eq!(transition.transform_level(10, 0), 10);
        assert_eq!(transition.transform_level(10, 10), 10);
        assert_eq!(transition.transform_level(10, 20), 10);
        assert_eq!(transition.transform_level(10, 30), 10);
        assert_eq!(transition.transform_level(10, 100), 11);
        assert_eq!(transition.transform_level(10, 130), 14);
        assert_eq!(transition.transform_level(10, 180), 19);
        assert_eq!(transition.transform_level(10, 280), 29);

        assert_eq!(transition.transform_level(15, 0), 15);
        assert_eq!(transition.transform_level(15, 10), 15);
        assert_eq!(transition.transform_level(15, 20), 15);
        assert_eq!(transition.transform_level(15, 30), 15);
        assert_eq!(transition.transform_level(15, 100), 16);
        assert_eq!(transition.transform_level(15, 130), 19);
        assert_eq!(transition.transform_level(15, 230), 29);

        assert_eq!(transition.transform_level(16, 0), 16);
        assert_eq!(transition.transform_level(16, 10), 16);
        assert_eq!(transition.transform_level(16, 20), 16);
        assert_eq!(transition.transform_level(16, 30), 16);
        assert_eq!(transition.transform_level(16, 110), 17);
        assert_eq!(transition.transform_level(16, 130), 19);
        assert_eq!(transition.transform_level(16, 230), 29);

        assert_eq!(transition.transform_level(18, 0), 18);
        assert_eq!(transition.transform_level(18, 10), 18);
        assert_eq!(transition.transform_level(18, 20), 18);
        assert_eq!(transition.transform_level(18, 30), 18);
        assert_eq!(transition.transform_level(18, 120), 18);
        assert_eq!(transition.transform_level(18, 130), 19);
        assert_eq!(transition.transform_level(18, 230), 29);

        assert_eq!(transition.transform_level(19, 0), 19);
        assert_eq!(transition.transform_level(19, 10), 19);
        assert_eq!(transition.transform_level(19, 20), 19);
        assert_eq!(transition.transform_level(19, 30), 19);
        assert_eq!(transition.transform_level(19, 130), 19);
        assert_eq!(transition.transform_level(19, 140), 20);
        assert_eq!(transition.transform_level(19, 230), 29);

        assert_eq!(transition.transform_level(26, 0), 26);
        assert_eq!(transition.transform_level(26, 10), 26);
        assert_eq!(transition.transform_level(26, 20), 26);
        assert_eq!(transition.transform_level(26, 30), 26);
        assert_eq!(transition.transform_level(26, 190), 26);
        assert_eq!(transition.transform_level(26, 200), 27);
        assert_eq!(transition.transform_level(26, 220), 29);
    }

    #[test]
    fn test_every_n_lines() {
        let transition = Transition::Every10Lines;

        assert_eq!(transition.transform_level(0, 0), 0);
        assert_eq!(transition.transform_level(0, 10), 1);
        assert_eq!(transition.transform_level(0, 20), 2);
        assert_eq!(transition.transform_level(0, 30), 3);
        assert_eq!(transition.transform_level(0, 180), 18);

        assert_eq!(transition.transform_level(18, 0), 18);
        assert_eq!(transition.transform_level(18, 10), 19);
        assert_eq!(transition.transform_level(18, 20), 20);
        assert_eq!(transition.transform_level(18, 30), 21);
        assert_eq!(transition.transform_level(18, 180), 36);
    }
}
