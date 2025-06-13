pub struct InputFrequency {
    count: usize,
    start_time: f32,
    end_time: f32,
    freq: f32,
}

impl InputFrequency {
    const TIMEOUT: f32 = 0.5;

    pub fn new() -> Self {
        Self {
            count: 0,
            start_time: 0.0,
            end_time: 0.0,
            freq: 0.0,
        }
    }

    pub fn reset_when_expired(&mut self, now: f32) {
        if self.end_time + Self::TIMEOUT < now {
            if self.count == 0 {
                self.freq = 0.0;
            } else if self.end_time == self.start_time {
                self.freq = 1.0;
            } else {
                self.freq = self.count as f32 / (self.end_time - self.start_time);
            }
            self.count = 0;
            self.start_time = now;
            self.end_time = now;
        }
    }

    pub fn increment(&mut self, now: f32) {
        if self.count == 0 {
            self.start_time = now;
        }

        self.count += 1;
        self.end_time = now;
    }

    pub fn freq(&self) -> f32 {
        self.freq
    }
}

impl Default for InputFrequency {
    fn default() -> Self {
        Self::new()
    }
}
