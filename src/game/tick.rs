use std::time::Duration;

const TICKS_PER_MICROSECOND: u64 = 60_098800; // NTSC 60.0988 Hz

const fn ticks_to_duration(count: u64) -> Duration {
    Duration::from_micros(count * 1000_000_000_000 / TICKS_PER_MICROSECOND)
}

pub fn duration_to_ticks(duration: Duration) -> u64 {
    (duration.as_secs_f64() * TICKS_PER_MICROSECOND as f64 / 1000_000.0).round() as u64
}

#[derive(Default)]
pub struct PressDownTick {
    duration: Duration,
}

impl PressDownTick {
    pub fn tick(&mut self, delta: Duration) {
        self.duration += delta;
    }

    pub fn consume(&mut self) -> bool {
        if self.duration >= Self::TRIGGER_TICK {
            self.duration -= Self::CONSUME_TICK;
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self) {
        self.duration = Duration::ZERO;
    }

    const TRIGGER_TICK: Duration = ticks_to_duration(3);
    const CONSUME_TICK: Duration = ticks_to_duration(2);
}

#[derive(Default)]
pub struct DelayAutoShiftTick {
    duration: Duration,
}

impl DelayAutoShiftTick {
    pub fn tick(&mut self, delta: Duration) {
        self.duration = (self.duration + delta).min(Self::TRIGGER_TICK);
    }

    pub fn consume(&mut self) -> bool {
        if self.duration >= Self::TRIGGER_TICK {
            self.duration -= Self::CONSUME_TICK;
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self) {
        self.duration = Duration::ZERO;
    }

    pub fn reset_max(&mut self) {
        self.duration = Self::TRIGGER_TICK;
    }

    pub fn duration(&self) -> Duration {
        self.duration
    }

    const TRIGGER_TICK: Duration = ticks_to_duration(16);
    const CONSUME_TICK: Duration = ticks_to_duration(6);
}

pub struct FallTick {
    duration: Duration,
    initial_entry_delay: bool,
}

impl FallTick {
    pub fn new() -> Self {
        Self {
            duration: Duration::ZERO,
            initial_entry_delay: true,
        }
    }

    pub fn tick(&mut self, delta: Duration) {
        self.duration += delta;
    }

    pub fn consume(&mut self, level: usize) -> bool {
        let mut trigger = ticks_to_duration(Self::get_trigger_tick(level));
        if self.initial_entry_delay {
            trigger += Self::INITIAL_ENTRY_DELAY_TICK;
        }

        if self.duration >= trigger {
            self.duration -= trigger;
            self.initial_entry_delay = false;
            return true;
        }

        false
    }

    pub fn reset(&mut self) {
        self.duration = Duration::ZERO;
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

impl Default for FallTick {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Default)]
pub struct LockdownDelayTick {
    duration: Duration,
    trigger: Duration,
}

impl LockdownDelayTick {
    pub fn tick(&mut self, delta: Duration) {
        self.duration += delta;
    }

    pub fn consume(&mut self) -> bool {
        if self.duration >= self.trigger {
            self.duration -= self.trigger;
            return true;
        }

        false
    }

    pub fn reset(&mut self, height: u64) {
        // lock in the bottom 2 rows are 10 ticks, and then with additional 2 ticks every 4 rows
        let ticks = (height + 2) / 4 * 2 + 10;
        self.duration = ticks_to_duration(ticks);
    }
}
