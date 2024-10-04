use std::{fmt::Display, time::Duration};

use bevy::prelude::*;
use num_traits::FromPrimitive;

#[derive(Default, Clone, Copy, FromPrimitive, Resource)]
pub enum FPSLimiter {
    #[default]
    Unlimited,
    Auto,
    F60,
    F144,
    F240,
    F360,
    F480,
}

impl FPSLimiter {
    pub fn enum_has_prev(&self) -> bool {
        <Self as FromPrimitive>::from_i64(*self as i64 - 1).is_some()
    }

    pub fn enum_has_next(&self) -> bool {
        <Self as FromPrimitive>::from_i64(*self as i64 + 1).is_some()
    }

    pub fn enum_prev(&mut self) -> bool {
        match FromPrimitive::from_i64(*self as i64 - 1) {
            Some(n) => {
                *self = n;
                true
            }
            None => false,
        }
    }

    pub fn enum_next(&mut self) -> bool {
        match FromPrimitive::from_i64(*self as i64 + 1) {
            Some(n) => {
                *self = n;
                true
            }
            None => false,
        }
    }

    pub fn get_limiter(&self) -> bevy_framepace::Limiter {
        let ft = |fps| Duration::from_secs_f32(1.0 / fps as f32);
        match self {
            FPSLimiter::Unlimited => bevy_framepace::Limiter::Off,
            FPSLimiter::Auto => bevy_framepace::Limiter::Auto,
            FPSLimiter::F60 => bevy_framepace::Limiter::Manual(ft(60)),
            FPSLimiter::F144 => bevy_framepace::Limiter::Manual(ft(144)),
            FPSLimiter::F240 => bevy_framepace::Limiter::Manual(ft(240)),
            FPSLimiter::F360 => bevy_framepace::Limiter::Manual(ft(360)),
            FPSLimiter::F480 => bevy_framepace::Limiter::Manual(ft(480)),
        }
    }
}

impl Display for FPSLimiter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FPSLimiter::Auto => f.write_str("AUTO"),
            FPSLimiter::Unlimited => f.write_str("UNLIMITED"),
            FPSLimiter::F60 => f.write_str("60 FPS"),
            FPSLimiter::F144 => f.write_str("144 FPS"),
            FPSLimiter::F240 => f.write_str("240 FPS"),
            FPSLimiter::F360 => f.write_str("360 FPS"),
            FPSLimiter::F480 => f.write_str("480 FPS"),
        }
    }
}
