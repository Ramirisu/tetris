use std::time::Duration;

use bevy::prelude::*;
use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, FromRepr};

use crate::enum_advance;

#[derive(Default, Clone, Copy, FromRepr, EnumIter, EnumCount, Resource)]
pub enum FPSLimiter {
    F240,
    F480,
    #[default]
    Unlimited,
}

enum_advance::enum_advance_derive!(FPSLimiter);

impl Into<bevy_framepace::Limiter> for FPSLimiter {
    fn into(self) -> bevy_framepace::Limiter {
        let ft = |fps| Duration::from_secs_f32(1.0 / fps as f32);
        match self {
            FPSLimiter::Unlimited => bevy_framepace::Limiter::Off,
            FPSLimiter::F240 => bevy_framepace::Limiter::Manual(ft(240)),
            FPSLimiter::F480 => bevy_framepace::Limiter::Manual(ft(480)),
        }
    }
}
