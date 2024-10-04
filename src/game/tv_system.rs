use std::{fmt::Display, time::Duration};

use num_traits::FromPrimitive;

#[derive(Default, Clone, Copy, PartialEq, Eq, FromPrimitive)]
pub enum TVSystem {
    #[default]
    NTSC,
    PAL,
}

impl TVSystem {
    pub fn enum_has_prev(&self) -> bool {
        <Self as FromPrimitive>::from_i64(*self as i64 - 1).is_some()
    }

    pub fn enum_has_next(&self) -> bool {
        <Self as FromPrimitive>::from_i64(*self as i64 + 1).is_some()
    }

    pub fn enum_prev(&mut self) -> bool {
        match FromPrimitive::from_i64(*self as i64 - 1) {
            Some(n) => {
                *self = n;
                true
            }
            None => false,
        }
    }

    pub fn enum_next(&mut self) -> bool {
        match FromPrimitive::from_i64(*self as i64 + 1) {
            Some(n) => {
                *self = n;
                true
            }
            None => false,
        }
    }

    pub const fn ticks_to_duration(&self, ticks: u64) -> Duration {
        self.subticks_to_duration(ticks * 1000)
    }

    pub const fn subticks_to_duration(&self, ticks: u64) -> Duration {
        Duration::from_micros(ticks * 1_000_000_000 / self.ticks_per_microsecond())
    }

    pub fn duration_to_ticks(&self, duration: Duration) -> u64 {
        (duration.as_secs_f64() * self.ticks_per_microsecond() as f64 / 1_000_000.0).round() as u64
    }

    const fn ticks_per_microsecond(&self) -> u64 {
        match self {
            TVSystem::NTSC => 60_000_000,
            TVSystem::PAL => 50_000_000,
        }
    }
}

impl Display for TVSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TVSystem::NTSC => f.write_str("NTSC"),
            TVSystem::PAL => f.write_str("PAL"),
        }
    }
}
