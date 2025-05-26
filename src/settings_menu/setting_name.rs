use crate::{
    game::{
        gravity::Gravity, invisible::Invisible, leveling::Leveling, linecap::Linecap,
        next_piece_hint::NextPieceHint, random::Random, score::Score, seeding::Seeding,
        transition::Transition, tv_system::TVSystem,
    },
    input::controller_mapping::ControllerMapping,
};

use super::{scale_factor::ScaleFactor, show_fps::ShowFPS};

#[cfg(all(not(target_arch = "wasm32"), feature = "fps_limiter"))]
use super::fps_limiter::FPSLimiter;

pub trait SettingName {
    fn name(&self) -> String;
}

impl SettingName for Transition {
    fn name(&self) -> String {
        match self {
            Transition::Classic => t!("tetris.settings.transition.classic"),
            Transition::Fixed => t!("tetris.settings.transition.fixed"),
            Transition::Every10Lines => t!("tetris.settings.transition.every10lines"),
            Transition::Every4Lines => t!("tetris.settings.transition.every4lines"),
        }
        .into()
    }
}

impl SettingName for Linecap {
    fn name(&self) -> String {
        match self {
            Linecap::Off => t!("tetris.settings.linecap.off"),
            Linecap::KillScreenX2 => t!("tetris.settings.linecap.killscreenx2"),
            Linecap::Halt => t!("tetris.settings.linecap.halt"),
        }
        .into()
    }
}

impl SettingName for Gravity {
    fn name(&self) -> String {
        match self {
            Gravity::Level => t!("tetris.settings.gravity.level"),
            Gravity::Locked => t!("tetris.settings.gravity.locked"),
        }
        .into()
    }
}

impl SettingName for Seeding {
    fn name(&self) -> String {
        match self {
            Seeding::System => t!("tetris.settings.seeding.system"),
            Seeding::Custom => t!("tetris.settings.seeding.custom"),
        }
        .into()
    }
}

impl SettingName for Random {
    fn name(&self) -> String {
        match self {
            Random::Uniform => t!("tetris.settings.random.uniform"),
            Random::Classic => t!("tetris.settings.random.classic"),
            Random::Modern => t!("tetris.settings.random.modern"),
        }
        .into()
    }
}

impl SettingName for Score {
    fn name(&self) -> String {
        match self {
            Score::Decimal => t!("tetris.settings.scoring.decimal"),
            Score::Classic => t!("tetris.settings.scoring.classic"),
            Score::Base36 => t!("tetris.settings.scoring.base36"),
        }
        .into()
    }
}

impl SettingName for Leveling {
    fn name(&self) -> String {
        match self {
            Leveling::Decimal => t!("tetris.settings.leveling.decimal"),
            Leveling::Classic => t!("tetris.settings.leveling.classic"),
        }
        .into()
    }
}

impl SettingName for TVSystem {
    fn name(&self) -> String {
        match self {
            TVSystem::NTSC => "NTSC",
            TVSystem::PAL => "PAL",
        }
        .into()
    }
}

impl SettingName for NextPieceHint {
    fn name(&self) -> String {
        match self {
            NextPieceHint::Off => t!("tetris.settings.next_piece_hint.off"),
            NextPieceHint::Classic => t!("tetris.settings.next_piece_hint.classic"),
            NextPieceHint::Modern => t!("tetris.settings.next_piece_hint.modern"),
        }
        .into()
    }
}

impl SettingName for Invisible {
    fn name(&self) -> String {
        match self {
            Invisible::Off => t!("tetris.settings.invisible.off"),
            Invisible::On => t!("tetris.settings.invisible.on"),
        }
        .into()
    }
}

#[cfg(all(not(target_arch = "wasm32"), feature = "fps_limiter"))]
impl SettingName for FPSLimiter {
    fn name(&self) -> String {
        match self {
            FPSLimiter::Unlimited => t!("tetris.settings.fps_limiter.unlimited"),
            FPSLimiter::F240 => t!("tetris.settings.fps_limiter.240fps"),
            FPSLimiter::F480 => t!("tetris.settings.fps_limiter.480fps"),
        }
        .into()
    }
}

impl SettingName for ShowFPS {
    fn name(&self) -> String {
        match self {
            ShowFPS::Off => t!("tetris.settings.show_fps.off"),
            ShowFPS::On => t!("tetris.settings.show_fps.on"),
        }
        .into()
    }
}

impl SettingName for ControllerMapping {
    fn name(&self) -> String {
        match self {
            ControllerMapping::MappingA => t!("tetris.settings.controller_mapping.mapping_a"),
            ControllerMapping::MappingB => t!("tetris.settings.controller_mapping.mapping_b"),
        }
        .into()
    }
}

impl SettingName for ScaleFactor {
    fn name(&self) -> String {
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
