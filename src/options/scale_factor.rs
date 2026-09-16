use bevy::prelude::*;
use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, FromRepr};

use crate::utility::enum_advance;

pub const WINDOW_WIDTH: f32 = 1440.0;
pub const WINDOW_HEIGHT: f32 = 1080.0;

#[derive(Default, Clone, Copy, FromRepr, EnumIter, EnumCount, Resource)]
pub enum ScaleFactor {
    S720,
    #[default]
    S1080,
    S1440,
    S1800,
    S2160,
    S2880,
    S3240,
    S4320,
}

enum_advance::enum_advance_derive!(ScaleFactor);

impl ScaleFactor {
    pub fn mul(&self) -> f32 {
        match self {
            ScaleFactor::S720 => 720.0 / WINDOW_HEIGHT,
            ScaleFactor::S1080 => 1080.0 / WINDOW_HEIGHT,
            ScaleFactor::S1440 => 1440.0 / WINDOW_HEIGHT,
            ScaleFactor::S1800 => 1800.0 / WINDOW_HEIGHT,
            ScaleFactor::S2160 => 2160.0 / WINDOW_HEIGHT,
            ScaleFactor::S2880 => 2880.0 / WINDOW_HEIGHT,
            ScaleFactor::S3240 => 3240.0 / WINDOW_HEIGHT,
            ScaleFactor::S4320 => 4320.0 / WINDOW_HEIGHT,
        }
    }
}
