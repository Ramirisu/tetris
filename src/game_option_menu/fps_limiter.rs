use std::{fmt::Display, time::Duration};

use bevy::prelude::*;
use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, FromRepr};

use crate::enum_advance;

#[derive(Default, Clone, Copy, FromRepr, EnumIter, EnumCount, Resource)]
pub enum FPSLimiter {
    #[default]
    Unlimited,
    F240,
    F480,
    F960,
}

enum_advance::enum_advance_derive!(FPSLimiter);

impl Into<bevy_framepace::Limiter> for FPSLimiter {
    fn into(self) -> bevy_framepace::Limiter {
        let ft = |fps| Duration::from_secs_f32(1.0 / fps as f32);
        match self {
            FPSLimiter::Unlimited => bevy_framepace::Limiter::Off,
            FPSLimiter::F240 => bevy_framepace::Limiter::Manual(ft(240)),
            FPSLimiter::F480 => bevy_framepace::Limiter::Manual(ft(480)),
            FPSLimiter::F960 => bevy_framepace::Limiter::Manual(ft(960)),
        }
    }
}

impl Display for FPSLimiter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FPSLimiter::Unlimited => f.write_str("UNLIMITED"),
            FPSLimiter::F240 => f.write_str("240 FPS"),
            FPSLimiter::F480 => f.write_str("480 FPS"),
            FPSLimiter::F960 => f.write_str("960 FPS"),
        }
    }
}
