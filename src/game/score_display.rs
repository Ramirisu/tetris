use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, FromRepr};

use crate::utility::enum_advance;

#[derive(Default, Clone, Copy, PartialEq, Eq, FromRepr, EnumIter, EnumCount)]
pub enum ScoreDisplay {
    #[default]
    Decimal,
    Classic,
    Base36,
}

enum_advance::enum_advance_derive!(ScoreDisplay);

impl ScoreDisplay {
    pub fn format(&self, score: usize) -> String {
        match self {
            ScoreDisplay::Decimal => format!("{:06}", score),
            ScoreDisplay::Classic => format!("{:06}", score.min(999999)),
            ScoreDisplay::Base36 => format!(
                "{}{:05}",
                Self::format_base36(score / 100000),
                score % 100000
            ),
        }
    }

    fn format_base36(mut value: usize) -> String {
        let mut s = String::new();

        while value > 0 {
            s.push(char::from_digit((value % 36) as u32, 36).unwrap());
            value /= 36;
        }

        if s.is_empty() {
            s.push_str("0");
        }

        s.chars().map(|c| c.to_ascii_uppercase()).rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal() {
        let sd = ScoreDisplay::Decimal;
        assert_eq!(sd.format(123), "000123");
        assert_eq!(sd.format(123456), "123456");
        assert_eq!(sd.format(999999), "999999");
        assert_eq!(sd.format(1000000), "1000000");
    }

    #[test]
    fn test_classic() {
        let sd = ScoreDisplay::Classic;
        assert_eq!(sd.format(123), "000123");
        assert_eq!(sd.format(123456), "123456");
        assert_eq!(sd.format(999999), "999999");
        assert_eq!(sd.format(1000000), "999999");
    }

    #[test]
    fn test_base36() {
        let sd = ScoreDisplay::Base36;
        assert_eq!(sd.format(123), "000123");
        assert_eq!(sd.format(123456), "123456");
        assert_eq!(sd.format(999999), "999999");
        assert_eq!(sd.format(1000000), "A00000");
        assert_eq!(sd.format(1100000), "B00000");
        assert_eq!(sd.format(1500000), "F00000");
        assert_eq!(sd.format(1600000), "G00000");
        assert_eq!(sd.format(3500000), "Z00000");
        assert_eq!(sd.format(3600000), "1000000");
        assert_eq!(sd.format(3700000), "1100000");
        assert_eq!(sd.format(36000000), "A000000");
        assert_eq!(sd.format(39500000), "AZ00000");
    }
}
