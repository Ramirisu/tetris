use std::time::Duration;

use super::{linecap::Linecap, tv_system::TVSystem};

#[derive(Clone, Copy)]
pub struct FallTimer {
    elapsed: Duration,
    threshold: Duration,
    linecap: Linecap,
    tv_system: TVSystem,
    initial_entry_delay: bool,
}

impl FallTimer {
    pub fn new(
        level: usize,
        linecap: Linecap,
        tv_system: TVSystem,
        initial_entry_delay: bool,
    ) -> Self {
        Self {
            elapsed: Duration::ZERO,
            threshold: Self::level_to_duration(level, linecap, tv_system),
            linecap,
            tv_system,
            initial_entry_delay,
        }
    }

    pub fn set_level(&mut self, level: usize) {
        *self = Self::new(level, self.linecap, self.tv_system, false);
    }

    pub fn tick(&mut self, delta: Duration) -> &mut Self {
        self.elapsed += delta;
        self
    }

    pub fn consume(&mut self) -> bool {
        let threshold = if self.initial_entry_delay {
            Self::get_initial_entry_delay(self.tv_system) + self.threshold
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

    fn level_to_duration(level: usize, linecap: Linecap, tv_system: TVSystem) -> Duration {
        match tv_system {
            TVSystem::NTSC => match level {
                0 => tv_system.ticks_to_duration(48),
                1 => tv_system.ticks_to_duration(43),
                2 => tv_system.ticks_to_duration(38),
                3 => tv_system.ticks_to_duration(33),
                4 => tv_system.ticks_to_duration(28),
                5 => tv_system.ticks_to_duration(23),
                6 => tv_system.ticks_to_duration(18),
                7 => tv_system.ticks_to_duration(13),
                8 => tv_system.ticks_to_duration(8),
                9 => tv_system.ticks_to_duration(6),
                10..13 => tv_system.ticks_to_duration(5),
                13..16 => tv_system.ticks_to_duration(4),
                16..19 => tv_system.ticks_to_duration(3),
                19..29 => tv_system.ticks_to_duration(2),
                29..39 => tv_system.ticks_to_duration(1),
                _ => match linecap {
                    Linecap::Off => tv_system.ticks_to_duration(1),
                    Linecap::SuperKillScreen => tv_system.subticks_to_duration(500),
                },
            },
            TVSystem::PAL => match level {
                0 => tv_system.ticks_to_duration(36),
                1 => tv_system.ticks_to_duration(32),
                2 => tv_system.ticks_to_duration(29),
                3 => tv_system.ticks_to_duration(25),
                4 => tv_system.ticks_to_duration(22),
                5 => tv_system.ticks_to_duration(18),
                6 => tv_system.ticks_to_duration(15),
                7 => tv_system.ticks_to_duration(11),
                8 => tv_system.ticks_to_duration(7),
                9 => tv_system.ticks_to_duration(5),
                10..13 => tv_system.ticks_to_duration(4),
                13..16 => tv_system.ticks_to_duration(3),
                16..19 => tv_system.ticks_to_duration(2),
                19..29 => tv_system.ticks_to_duration(1),
                _ => match linecap {
                    Linecap::Off => tv_system.ticks_to_duration(1),
                    Linecap::SuperKillScreen => tv_system.subticks_to_duration(500),
                },
            },
        }
    }

    fn get_initial_entry_delay(tv_system: TVSystem) -> Duration {
        match tv_system {
            TVSystem::NTSC => tv_system.ticks_to_duration(96),
            TVSystem::PAL => tv_system.ticks_to_duration(72), // TODO: is this correct?
        }
    }
}

pub struct PressDownTimer {
    elapsed: Duration,
    threshold: Duration,
}

impl PressDownTimer {
    pub fn new(tv_system: TVSystem) -> Self {
        Self {
            elapsed: Duration::ZERO,
            threshold: Self::get_threshold(tv_system),
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

    pub fn reset(&mut self) {
        self.elapsed = Duration::ZERO;
    }

    fn get_threshold(tv_system: TVSystem) -> Duration {
        match tv_system {
            TVSystem::NTSC => tv_system.ticks_to_duration(2),
            TVSystem::PAL => tv_system.ticks_to_duration(2), // TODO: is this correct?
        }
    }
}

pub struct DelayAutoShiftTimer {
    elapsed: Duration,
    threshold: Duration,
    consumption: Duration,
}

impl DelayAutoShiftTimer {
    pub fn new(tv_system: TVSystem) -> Self {
        Self {
            elapsed: Duration::ZERO,
            threshold: Self::get_threshold(tv_system),
            consumption: Self::get_consumption(tv_system),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.elapsed
    }

    pub fn tick(&mut self, delta: Duration) -> &mut Self {
        self.elapsed += delta;
        self
    }

    pub fn consume(&mut self) -> bool {
        if self.elapsed >= self.threshold {
            self.elapsed -= self.consumption;
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self) {
        self.elapsed = Duration::ZERO;
    }

    pub fn charge(&mut self) {
        self.elapsed = self.threshold;
    }

    pub fn is_active(&self) -> bool {
        self.elapsed >= (self.threshold - self.consumption)
    }

    fn get_threshold(tv_system: TVSystem) -> Duration {
        match tv_system {
            TVSystem::NTSC => tv_system.ticks_to_duration(16),
            TVSystem::PAL => tv_system.ticks_to_duration(12),
        }
    }

    fn get_consumption(tv_system: TVSystem) -> Duration {
        match tv_system {
            TVSystem::NTSC => tv_system.ticks_to_duration(6),
            TVSystem::PAL => tv_system.ticks_to_duration(4),
        }
    }
}

pub struct LineClearTimer {
    elapsed: Duration,
    threshold: Duration,
}

impl LineClearTimer {
    pub fn new(phase: u32, tv_system: TVSystem) -> Self {
        Self {
            elapsed: Duration::ZERO,
            threshold: Self::get_threshold(tv_system) / phase,
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

    fn get_threshold(tv_system: TVSystem) -> Duration {
        match tv_system {
            TVSystem::NTSC => tv_system.ticks_to_duration(18),
            TVSystem::PAL => tv_system.ticks_to_duration(18), // TODO: is this correct?
        }
    }
}

pub struct EntryDelayTimer {
    elapsed: Duration,
    threshold: Duration,
}

impl EntryDelayTimer {
    pub fn new(height: u64, tv_system: TVSystem) -> Self {
        Self {
            elapsed: Duration::ZERO,
            threshold: Self::height_to_duration(height, tv_system),
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

    fn height_to_duration(height: u64, tv_system: TVSystem) -> Duration {
        match tv_system {
            TVSystem::NTSC => {
                // lock in the bottom 2 rows are 10 ticks, and then with additional 2 ticks every 4 rows
                let ticks = (height + 2) / 4 * 2 + 10;
                tv_system.ticks_to_duration(ticks)
            }
            TVSystem::PAL => {
                // TODO: is this correct?
                let ticks = (height + 2) / 4 * 2 + 10;
                tv_system.ticks_to_duration(ticks)
            }
        }
    }
}
