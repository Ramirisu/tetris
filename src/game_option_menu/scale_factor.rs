use std::fmt::Display;

use bevy::prelude::*;
use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, FromRepr};

use crate::enum_advance;

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

impl Display for ScaleFactor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScaleFactor::S720 => f.write_str("0.66 (720P)"),
            ScaleFactor::S1080 => f.write_str("1.00 (1080P)"),
            ScaleFactor::S1440 => f.write_str("1.33 (1440P)"),
            ScaleFactor::S1800 => f.write_str("1.66 (1800P)"),
            ScaleFactor::S2160 => f.write_str("2.00 (2160P)"),
            ScaleFactor::S2880 => f.write_str("2.66 (2880P)"),
            ScaleFactor::S3240 => f.write_str("3.00 (3240P)"),
            ScaleFactor::S4320 => f.write_str("4.00 (4320P)"),
        }
    }
}
