use std::time::Duration;

use super::tick::ticks_to_duration;

#[derive(Default)]
pub struct GameTimer {
    duration: Duration,
}

impl GameTimer {
    pub fn tick(&mut self, delta: Duration) {
        self.duration += delta;
    }

    pub fn commit(&mut self, threshold: Duration) -> bool {
        if self.duration >= threshold {
            self.duration -= threshold;
            true
        } else {
            false
        }
    }
}

#[derive(Default)]
pub struct PressDownTimer {
    duration: Duration,
}

impl PressDownTimer {
    pub fn tick(&mut self, delta: Duration) {
        self.duration += delta;
    }

    pub fn commit(&mut self) -> bool {
        if self.duration >= Self::TRIGGER_TICK {
            self.duration -= Self::TRIGGER_TICK;
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self) {
        self.duration = Duration::ZERO;
    }

    const TRIGGER_TICK: Duration = ticks_to_duration(2);
}

#[derive(Default)]
pub struct DelayAutoShiftTimer {
    duration: Duration,
}

impl DelayAutoShiftTimer {
    pub fn tick(&mut self, delta: Duration) {
        self.duration = (self.duration + delta).min(Self::TRIGGER_TICK);
    }

    pub fn commit(&mut self) -> bool {
        if self.duration >= Self::TRIGGER_TICK {
            self.duration -= Self::COMMIT_TICK;
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
    const COMMIT_TICK: Duration = ticks_to_duration(6);
}
