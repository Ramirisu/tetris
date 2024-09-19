use std::time::Duration;

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
    pub fn enum_prev(&mut self) -> Option<Self> {
        FromPrimitive::from_i8(*self as i8 - 1).map(|n| std::mem::replace(self, n))
    }

    pub fn enum_next(&mut self) -> Option<Self> {
        FromPrimitive::from_i8(*self as i8 + 1).map(|n| std::mem::replace(self, n))
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
