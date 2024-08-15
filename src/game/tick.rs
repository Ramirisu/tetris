use std::time::Duration;

const TICKS_PER_MICROSECOND: u64 = 60_098800; // NTSC 60.0988 Hz

pub const fn ticks_to_duration(count: u64) -> Duration {
    Duration::from_micros(count * 1000_000_000_000 / TICKS_PER_MICROSECOND)
}

pub fn duration_to_ticks(duration: Duration) -> u64 {
    (duration.as_secs_f64() * TICKS_PER_MICROSECOND as f64 / 1000_000.0).round() as u64
}

pub struct FallTick {
    threshold: Duration,
    initial_entry_delay: bool,
}

impl FallTick {
    pub fn new(level: usize, initial_entry_delay: bool) -> Self {
        Self {
            threshold: ticks_to_duration(Self::get_trigger_tick(level)),
            initial_entry_delay,
        }
    }

    pub fn threshold(&self) -> Duration {
        if self.initial_entry_delay {
            Self::INITIAL_ENTRY_DELAY_TICK + self.threshold
        } else {
            self.threshold
        }
    }

    fn get_trigger_tick(level: usize) -> u64 {
        const TABLE: [u64; 29] = [
            48, 43, 38, 33, 28, 23, 18, 13, 8, 6, 5, 5, 5, 4, 4, 4, 3, 3, 3, 2, 2, 2, 2, 2, 2, 2,
            2, 2, 2,
        ];

        if level < 29 {
            TABLE[level]
        } else {
            1
        }
    }

    const INITIAL_ENTRY_DELAY_TICK: Duration = ticks_to_duration(96);
}

#[derive(Default)]
pub struct LineClearTick {
    threshold: Duration,
}

impl LineClearTick {
    pub fn new(phase: usize) -> Self {
        Self {
            threshold: Self::TOTAL_DRUATION / phase as u32,
        }
    }

    pub fn threshold(&self) -> Duration {
        self.threshold
    }

    const TOTAL_DRUATION: Duration = ticks_to_duration(18);
}

#[derive(Default)]
pub struct EntryDelayTick {
    threshold: Duration,
}

impl EntryDelayTick {
    pub fn new(height: u64) -> Self {
        // lock in the bottom 2 rows are 10 ticks, and then with additional 2 ticks every 4 rows
        let ticks = (height + 2) / 4 * 2 + 10;
        Self {
            threshold: ticks_to_duration(ticks),
        }
    }

    pub fn threshold(&self) -> Duration {
        self.threshold
    }
}
