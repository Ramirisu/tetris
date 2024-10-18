use std::fmt::Display;

use crate::enum_iter;

#[derive(Default, Clone, Copy, PartialEq, Eq, FromPrimitive)]
pub enum Scoring {
    #[default]
    Decimal,
    Classic,
    Base36,
}

enum_iter::enum_iter_derive!(Scoring);

impl Scoring {
    pub fn format(&self, score: usize) -> String {
        match self {
            Scoring::Decimal => format!("{:06}", score),
            Scoring::Classic => {
                if score > 999999 {
                    "999999".into()
                } else {
                    format!("{:06}", score)
                }
            }
            Scoring::Base36 => format!(
                "{}{:05}",
                Self::format_base36(score / 100000),
                score % 100000
            ),
        }
    }

    pub fn format_base36(mut value: usize) -> String {
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

impl Display for Scoring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Scoring::Decimal => f.write_str("DECIMAL"),
            Scoring::Classic => f.write_str("CLASSIC"),
            Scoring::Base36 => f.write_str("BASE36"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal() {
        let scoring = Scoring::Decimal;
        assert_eq!(scoring.format(123), "000123");
        assert_eq!(scoring.format(123456), "123456");
        assert_eq!(scoring.format(999999), "999999");
        assert_eq!(scoring.format(1000000), "1000000");
    }

    #[test]
    fn test_classic() {
        let scoring = Scoring::Classic;
        assert_eq!(scoring.format(123), "000123");
        assert_eq!(scoring.format(123456), "123456");
        assert_eq!(scoring.format(999999), "999999");
        assert_eq!(scoring.format(1000000), "999999");
    }

    #[test]
    fn test_base36() {
        let scoring = Scoring::Base36;
        assert_eq!(scoring.format(123), "000123");
        assert_eq!(scoring.format(123456), "123456");
        assert_eq!(scoring.format(999999), "999999");
        assert_eq!(scoring.format(1000000), "A00000");
        assert_eq!(scoring.format(1100000), "B00000");
        assert_eq!(scoring.format(1500000), "F00000");
        assert_eq!(scoring.format(1600000), "G00000");
        assert_eq!(scoring.format(3500000), "Z00000");
        assert_eq!(scoring.format(3600000), "1000000");
        assert_eq!(scoring.format(3700000), "1100000");
        assert_eq!(scoring.format(36000000), "A000000");
        assert_eq!(scoring.format(39500000), "AZ00000");
    }
}
