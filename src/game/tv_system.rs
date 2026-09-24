use std::time::Duration;

use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, FromRepr};

#[derive(Default, Clone, Copy, PartialEq, Eq, FromRepr, EnumIter, EnumCount, tetris_macros::EnumAdvance)]
pub enum TVSystem {
    #[default]
    NTSC,
    PAL,
}


impl TVSystem {
    pub const fn ticks_to_duration(self, ticks: u64) -> Duration {
        self.subticks_to_duration(ticks * 1_000)
    }

    pub const fn subticks_to_duration(self, subticks: u64) -> Duration {
        Duration::from_nanos(self.duration_per_tick().as_nanos() as u64 * subticks / 1_000)
    }

    pub fn duration_to_ticks(self, duration: Duration) -> u64 {
        (duration.as_secs_f64() / self.duration_per_tick().as_secs_f64()).round() as u64
    }

    const fn duration_per_tick(self) -> Duration {
        match self {
            TVSystem::NTSC => Duration::from_nanos(1_000_000_000 * 10000 / 600988), // 60.0988 fps
            TVSystem::PAL => Duration::from_nanos(1_000_000_000 * 10000 / 500070),  // 50.0070 fps
        }
    }
}
