use crate::{
    game::{
        gravity::Gravity, invisible::Invisible, linecap::Linecap, next_piece_hint::NextPieceHint,
        scoring::Scoring, seeding::Seeding, transition::Transition, tv_system::TVSystem,
    },
    input::controller_mapping::ControllerMapping,
};

use super::{fps_limiter::FPSLimiter, scale_factor::ScaleFactor, show_fps::ShowFPS};

pub trait GameOption {
    fn desc(&self) -> String;
}

impl GameOption for Transition {
    fn desc(&self) -> String {
        match self {
            Transition::Classic => "CLASSIC",
            Transition::Fixed => "FIXED",
            Transition::Every10Lines => "EVERY 10 LINES",
            Transition::Every4Lines => "EVERY 4 LINES",
        }
        .into()
    }
}

impl GameOption for Linecap {
    fn desc(&self) -> String {
        match self {
            Linecap::Off => "OFF",
            Linecap::SuperKillScreen => "SUPER KILL SCREEN",
        }
        .into()
    }
}

impl GameOption for Gravity {
    fn desc(&self) -> String {
        match self {
            Gravity::Level => "LEVEL",
            Gravity::Locked => "LOCKED",
        }
        .into()
    }
}

impl GameOption for Seeding {
    fn desc(&self) -> String {
        match self {
            Seeding::System => "SYSTEM",
            Seeding::Custom => "CUSTOM",
        }
        .into()
    }
}

impl GameOption for Scoring {
    fn desc(&self) -> String {
        match self {
            Scoring::Decimal => "DECIMAL",
            Scoring::Classic => "CLASSIC",
            Scoring::Base36 => "BASE36",
        }
        .into()
    }
}

impl GameOption for TVSystem {
    fn desc(&self) -> String {
        match self {
            TVSystem::NTSC => "NTSC",
            TVSystem::PAL => "PAL",
        }
        .into()
    }
}

impl GameOption for NextPieceHint {
    fn desc(&self) -> String {
        match self {
            NextPieceHint::Off => "OFF",
            NextPieceHint::Classic => "CLASSIC",
            NextPieceHint::Modern => "MODERN",
        }
        .into()
    }
}

impl GameOption for Invisible {
    fn desc(&self) -> String {
        match self {
            Invisible::Off => "OFF",
            Invisible::On => "ON",
        }
        .into()
    }
}

impl GameOption for FPSLimiter {
    fn desc(&self) -> String {
        match self {
            FPSLimiter::Unlimited => "UNLIMITED",
            FPSLimiter::F240 => "240 FPS",
            FPSLimiter::F480 => "480 FPS",
        }
        .into()
    }
}

impl GameOption for ShowFPS {
    fn desc(&self) -> String {
        match self {
            ShowFPS::Off => "OFF",
            ShowFPS::Auto => "AUTO",
            ShowFPS::On => "ON",
        }
        .into()
    }
}

impl GameOption for ControllerMapping {
    fn desc(&self) -> String {
        match self {
            ControllerMapping::MappingA => "MAPPING A",
            ControllerMapping::MappingB => "MAPPING B",
        }
        .into()
    }
}

impl GameOption for ScaleFactor {
    fn desc(&self) -> String {
        match self {
            ScaleFactor::S720 => "0.66 (720P)",
            ScaleFactor::S1080 => "1.00 (1080P)",
            ScaleFactor::S1440 => "1.33 (1440P)",
            ScaleFactor::S1800 => "1.66 (1800P)",
            ScaleFactor::S2160 => "2.00 (2160P)",
            ScaleFactor::S2880 => "2.66 (2880P)",
            ScaleFactor::S3240 => "3.00 (3240P)",
            ScaleFactor::S4320 => "4.00 (4320P)",
        }
        .into()
    }
}
