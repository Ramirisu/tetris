use std::time::Duration;

use super::drop_speed::DropSpeed;

const TICKS_PER_MICROSECOND: u64 = 60_000_000;

pub const fn ticks_to_duration(count: u64) -> Duration {
    Duration::from_micros(count * 1_000_000_000_000 / TICKS_PER_MICROSECOND)
}

// 1 tick = 1000 subticks
pub const fn sub_ticks_to_duration(count: u64) -> Duration {
    Duration::from_micros(count * 1_000_000_000 / TICKS_PER_MICROSECOND)
}

pub fn duration_to_ticks(duration: Duration) -> u64 {
    (duration.as_secs_f64() * TICKS_PER_MICROSECOND as f64 / 1_000_000.0).round() as u64
}

pub struct FallTick {
    threshold: Duration,
    initial_entry_delay: bool,
    lv39_linecap: bool,
    drop_speed: DropSpeed,
}

impl FallTick {
    pub fn new(level: usize, lv39_linecap: bool, drop_speed: DropSpeed) -> Self {
        Self {
            threshold: Self::get_trigger_tick(level, lv39_linecap),
            initial_entry_delay: true,
            lv39_linecap,
            drop_speed,
        }
    }

    pub fn set_level(&mut self, level: usize) {
        self.initial_entry_delay = false;
        match self.drop_speed {
            DropSpeed::Classic => self.threshold = Self::get_trigger_tick(level, self.lv39_linecap),
            DropSpeed::Locked => (),
        }
    }

    pub fn threshold(&self) -> Duration {
        if self.initial_entry_delay {
            Self::INITIAL_ENTRY_DELAY_TICK + self.threshold
        } else {
            self.threshold
        }
    }

    fn get_trigger_tick(level: usize, lv39_linecap: bool) -> Duration {
        match level {
            0 => ticks_to_duration(48),
            1 => ticks_to_duration(43),
            2 => ticks_to_duration(38),
            3 => ticks_to_duration(33),
            4 => ticks_to_duration(28),
            5 => ticks_to_duration(23),
            6 => ticks_to_duration(18),
            7 => ticks_to_duration(13),
            8 => ticks_to_duration(8),
            9 => ticks_to_duration(6),
            10..13 => ticks_to_duration(5),
            13..16 => ticks_to_duration(4),
            16..19 => ticks_to_duration(3),
            19..29 => ticks_to_duration(2),
            29..39 => ticks_to_duration(1),
            _ => {
                if lv39_linecap {
                    sub_ticks_to_duration(500)
                } else {
                    ticks_to_duration(1)
                }
            }
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
