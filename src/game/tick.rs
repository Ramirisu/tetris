use std::time::Duration;

const TICKS_PER_MICROSECOND: u64 = 60_000_000;

pub const fn ticks_to_duration(count: u64) -> Duration {
    Duration::from_micros(count * 1_000_000_000_000 / TICKS_PER_MICROSECOND)
}

// 1 tick = 1000 subticks
pub const fn subticks_to_duration(count: u64) -> Duration {
    Duration::from_micros(count * 1_000_000_000 / TICKS_PER_MICROSECOND)
}

pub fn duration_to_ticks(duration: Duration) -> u64 {
    (duration.as_secs_f64() * TICKS_PER_MICROSECOND as f64 / 1_000_000.0).round() as u64
}
