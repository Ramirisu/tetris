use std::time::Duration;

use num_traits::FromPrimitive;

#[derive(Default, Clone, Copy, PartialEq, Eq, FromPrimitive)]
pub enum TVSystem {
    #[default]
    NTSC,
    PAL,
}

impl TVSystem {
    pub fn enum_prev(&mut self) -> Option<Self> {
        FromPrimitive::from_i8(*self as i8 - 1).map(|n| std::mem::replace(self, n))
    }

    pub fn enum_next(&mut self) -> Option<Self> {
        FromPrimitive::from_i8(*self as i8 + 1).map(|n| std::mem::replace(self, n))
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
