use std::time::Duration;

use super::{gravity::Gravity, level::Level, linecap::Linecap, tv_system::TVSystem};

#[derive(Clone, Copy)]
pub struct SoftDropTimer {
    elapsed: Duration,
    threshold: Duration,
    linecap: Linecap,
    linecap_level: Level,
    gravity: Gravity,
    tv_system: TVSystem,
    initial_entry_delay: bool,
}

impl SoftDropTimer {
    pub fn new(
        start_level: Level,
        linecap: Linecap,
        linecap_level: Level,
        gravity: Gravity,
        tv_system: TVSystem,
        initial_entry_delay: bool,
    ) -> Self {
        Self {
            elapsed: Duration::ZERO,
            threshold: Self::level_to_duration(start_level, linecap_level, linecap, tv_system),
            linecap,
            linecap_level,
            gravity,
            tv_system,
            initial_entry_delay,
        }
    }

    pub fn set_level(&mut self, level: Level) {
        self.elapsed = Duration::ZERO;
        match self.gravity {
            Gravity::Level => {
                self.threshold = Self::level_to_duration(
                    level,
                    self.linecap_level,
                    self.linecap,
                    self.tv_system,
                );
            }
            Gravity::Locked => (),
        }
        self.initial_entry_delay = false;
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

    fn level_to_duration(
        level: Level,
        linecap_level: Level,
        linecap: Linecap,
        tv_system: TVSystem,
    ) -> Duration {
        if linecap == Linecap::KillScreenX2 && level >= linecap_level {
            return tv_system.subticks_to_duration(500);
        }

        match tv_system {
            TVSystem::NTSC => match level.0 {
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
                _ => tv_system.ticks_to_duration(1),
            },
            TVSystem::PAL => match level.0 {
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
                _ => tv_system.ticks_to_duration(1),
            },
        }
    }

    fn get_initial_entry_delay(tv_system: TVSystem) -> Duration {
        match tv_system {
            TVSystem::NTSC => tv_system.ticks_to_duration(96),
            TVSystem::PAL => tv_system.ticks_to_duration(72), // TODO: correct?
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
            TVSystem::PAL => tv_system.ticks_to_duration(2), // TODO: correct?
        }
    }
}

pub struct DelayAutoShiftTimer {
    elapsed: Duration,
    tv_system: TVSystem,
}

impl DelayAutoShiftTimer {
    pub fn new(tv_system: TVSystem) -> Self {
        Self {
            elapsed: Duration::ZERO,
            tv_system,
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
        if self.elapsed >= self.get_threshold() {
            self.elapsed = self.get_active_threshold();
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self) {
        self.elapsed = Duration::ZERO;
    }

    pub fn charge(&mut self) {
        self.elapsed = self.get_threshold();
    }

    pub fn is_active(&self) -> bool {
        self.elapsed >= self.get_active_threshold()
    }

    pub fn get_threshold_ticks(&self) -> u64 {
        match self.tv_system {
            TVSystem::NTSC => 16,
            TVSystem::PAL => 12,
        }
    }

    pub fn get_active_threshold_ticks(&self) -> u64 {
        match self.tv_system {
            TVSystem::NTSC => 10,
            TVSystem::PAL => 8,
        }
    }

    fn get_threshold(&self) -> Duration {
        self.tv_system.ticks_to_duration(self.get_threshold_ticks())
    }

    fn get_active_threshold(&self) -> Duration {
        self.tv_system
            .ticks_to_duration(self.get_active_threshold_ticks())
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
            TVSystem::PAL => tv_system.ticks_to_duration(18), // TODO: correct?
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
                // TODO: correct?
                let ticks = (height + 2) / 4 * 2 + 10;
                tv_system.ticks_to_duration(ticks)
            }
        }
    }
}
