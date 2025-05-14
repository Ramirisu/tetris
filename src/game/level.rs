use std::ops::{AddAssign, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Level(pub usize);

impl Level {
    pub fn mod_palette_cycle(&self) -> Level {
        const CYCLE: usize = 256;
        Level(self.0 % CYCLE)
    }
}

impl AddAssign<usize> for Level {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

impl SubAssign<usize> for Level {
    fn sub_assign(&mut self, rhs: usize) {
        self.0 -= rhs;
    }
}

impl PartialEq<usize> for Level {
    fn eq(&self, other: &usize) -> bool {
        self.0.eq(other)
    }
}

impl PartialOrd<usize> for Level {
    fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
