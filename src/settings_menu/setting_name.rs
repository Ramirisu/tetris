use crate::{
    game::{
        gravity::Gravity, invisible::Invisible, linecap::Linecap, next_piece_hint::NextPieceHint,
        scoring::Scoring, seeding::Seeding, transition::Transition, tv_system::TVSystem,
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
            Transition::Classic => t!("tetris.game_option.transition.classic"),
            Transition::Fixed => t!("tetris.game_option.transition.fixed"),
            Transition::Every10Lines => t!("tetris.game_option.transition.every10lines"),
            Transition::Every4Lines => t!("tetris.game_option.transition.every4lines"),
        }
        .into()
    }
}

impl SettingName for Linecap {
    fn name(&self) -> String {
        match self {
            Linecap::Off => t!("tetris.game_option.linecap.off"),
            Linecap::KillScreenX2 => t!("tetris.game_option.linecap.killscreenx2"),
        }
        .into()
    }
}

impl SettingName for Gravity {
    fn name(&self) -> String {
        match self {
            Gravity::Level => t!("tetris.game_option.gravity.level"),
            Gravity::Locked => t!("tetris.game_option.gravity.locked"),
        }
        .into()
    }
}

impl SettingName for Seeding {
    fn name(&self) -> String {
        match self {
            Seeding::System => t!("tetris.game_option.seeding.system"),
            Seeding::Custom => t!("tetris.game_option.seeding.custom"),
        }
        .into()
    }
}

impl SettingName for Scoring {
    fn name(&self) -> String {
        match self {
            Scoring::Decimal => t!("tetris.game_option.scoring.decimal"),
            Scoring::Classic => t!("tetris.game_option.scoring.classic"),
            Scoring::Base36 => t!("tetris.game_option.scoring.base36"),
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
            NextPieceHint::Off => t!("tetris.game_option.next_piece_hint.off"),
            NextPieceHint::Classic => t!("tetris.game_option.next_piece_hint.classic"),
            NextPieceHint::Modern => t!("tetris.game_option.next_piece_hint.modern"),
        }
        .into()
    }
}

impl SettingName for Invisible {
    fn name(&self) -> String {
        match self {
            Invisible::Off => t!("tetris.game_option.invisible.off"),
            Invisible::On => t!("tetris.game_option.invisible.on"),
        }
        .into()
    }
}

#[cfg(all(not(target_arch = "wasm32"), feature = "fps_limiter"))]
impl SettingName for FPSLimiter {
    fn name(&self) -> String {
        match self {
            FPSLimiter::Unlimited => t!("tetris.game_option.fps_limiter.unlimited"),
            FPSLimiter::F240 => t!("tetris.game_option.fps_limiter.240fps"),
            FPSLimiter::F480 => t!("tetris.game_option.fps_limiter.480fps"),
        }
        .into()
    }
}

impl SettingName for ShowFPS {
    fn name(&self) -> String {
        match self {
            ShowFPS::Off => t!("tetris.game_option.show_fps.off"),
            ShowFPS::Auto => t!("tetris.game_option.show_fps.auto"),
            ShowFPS::On => t!("tetris.game_option.show_fps.on"),
        }
        .into()
    }
}

impl SettingName for ControllerMapping {
    fn name(&self) -> String {
        match self {
            ControllerMapping::MappingA => t!("tetris.game_option.controller_mapping.mapping_a"),
            ControllerMapping::MappingB => t!("tetris.game_option.controller_mapping.mapping_b"),
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
