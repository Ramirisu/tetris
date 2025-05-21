pub fn flicker(elapsed_sec: f32, period_sec: f32) -> f32 {
    (((std::f32::consts::TAU * elapsed_sec / period_sec).sin() / 2.0) + 0.5).clamp(0.0, 1.0)
}
