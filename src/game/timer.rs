use std::time::Duration;

use super::{
    linecap::Linecap,
    tick::{subticks_to_duration, ticks_to_duration},
};

#[derive(Clone, Copy)]
pub struct FallTimer {
    elapsed: Duration,
    threshold: Duration,
    linecap: Linecap,
    initial_entry_delay: bool,
}

impl FallTimer {
    pub fn new(level: usize, linecap: Linecap, initial_entry_delay: bool) -> Self {
        Self {
            elapsed: Duration::ZERO,
            threshold: Self::level_to_duration(level, linecap),
            linecap,
            initial_entry_delay,
        }
    }

    pub fn set_level(&mut self, level: usize) {
        *self = Self::new(level, self.linecap, false);
    }

    pub fn tick(&mut self, delta: Duration) -> &mut Self {
        self.elapsed += delta;
        self
    }

    pub fn consume(&mut self) -> bool {
        let threshold = if self.initial_entry_delay {
            Self::INITIAL_ENTRY_DELAY + self.threshold
        } else {
            self.threshold
        };

        if self.elapsed >= threshold {
            self.elapsed -= threshold;
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self) {
        self.elapsed = Duration::ZERO;
    }

    fn level_to_duration(level: usize, linecap: Linecap) -> Duration {
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
            _ => match linecap {
                Linecap::None => ticks_to_duration(1),
                Linecap::KillScreenX2 => subticks_to_duration(500),
            },
        }
    }

    const INITIAL_ENTRY_DELAY: Duration = ticks_to_duration(96);
}

#[derive(Default)]
pub struct PressDownTimer {
    elapsed: Duration,
}

impl PressDownTimer {
    pub fn tick(&mut self, delta: Duration) -> &mut Self {
        self.elapsed += delta;
        self
    }

    pub fn consume(&mut self) -> bool {
        if self.elapsed >= Self::THRESHOLD {
            self.elapsed -= Self::THRESHOLD;
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self) {
        self.elapsed = Duration::ZERO;
    }

    const THRESHOLD: Duration = ticks_to_duration(2);
}

#[derive(Default)]
pub struct DelayAutoShiftTimer {
    elapsed: Duration,
}

impl DelayAutoShiftTimer {
    pub fn elapsed(&self) -> Duration {
        self.elapsed
    }

    pub fn tick(&mut self, delta: Duration) -> &mut Self {
        self.elapsed += delta;
        self
    }

    pub fn consume(&mut self) -> bool {
        if self.elapsed >= Self::THRESHOLD {
            self.elapsed -= Self::CONSUME;
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self) {
        self.elapsed = Duration::ZERO;
    }

    pub fn charge(&mut self) {
        self.elapsed = Self::THRESHOLD;
    }

    const THRESHOLD: Duration = ticks_to_duration(16);
    const CONSUME: Duration = ticks_to_duration(6);
}

pub struct LineClearTimer {
    elapsed: Duration,
    threshold: Duration,
}

impl LineClearTimer {
    pub fn new(phase: u32) -> Self {
        Self {
            elapsed: Duration::ZERO,
            threshold: Self::THRESHOLD / phase,
        }
    }
    pub fn tick(&mut self, delta: Duration) -> &mut Self {
        self.elapsed += delta;
        self
    }

    pub fn consume(&mut self) -> bool {
        if self.elapsed >= self.threshold {
            self.elapsed -= self.threshold;
            true
        } else {
            false
        }
    }

    const THRESHOLD: Duration = ticks_to_duration(18);
}

pub struct EntryDelayTimer {
    elapsed: Duration,
    threshold: Duration,
}

impl EntryDelayTimer {
    pub fn new(height: u64) -> Self {
        Self {
            elapsed: Duration::ZERO,
            threshold: Self::height_to_duration(height),
        }
    }
    pub fn tick(&mut self, delta: Duration) -> &mut Self {
        self.elapsed += delta;
        self
    }

    pub fn consume(&mut self) -> bool {
        if self.elapsed >= self.threshold {
            self.elapsed -= self.threshold;
            true
        } else {
            false
        }
    }

    fn height_to_duration(height: u64) -> Duration {
        // lock in the bottom 2 rows are 10 ticks, and then with additional 2 ticks every 4 rows
        let ticks = (height + 2) / 4 * 2 + 10;
        ticks_to_duration(ticks)
    }
}
