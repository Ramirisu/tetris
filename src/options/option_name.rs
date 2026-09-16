use crate::{
    game::{
        gravity::Gravity, invisible::Invisible, level_display::LevelDisplay, linecap::Linecap,
        next_piece_hint::NextPieceHint, random::Random, score_display::ScoreDisplay,
        seeding::Seeding, tetris_flash::TetrisFlash, transition::Transition, tv_system::TVSystem,
    },
    input::controller_mapping::ControllerMapping,
};

use super::{scale_factor::ScaleFactor, show_fps::ShowFPS};

#[cfg(all(not(target_arch = "wasm32"), feature = "fps_limiter"))]
use super::fps_limiter::FPSLimiter;

#[cfg(not(target_arch = "wasm32"))]
use super::window_mode::WindowMode;

pub trait OptionName {
    fn name(&self) -> String;
}

impl OptionName for Transition {
    fn name(&self) -> String {
        match self {
            Transition::Classic => t!("tetris.options.transition.classic"),
            Transition::Fixed => t!("tetris.options.transition.fixed"),
            Transition::Every10Lines => t!("tetris.options.transition.every10lines"),
            Transition::Every4Lines => t!("tetris.options.transition.every4lines"),
        }
        .into()
    }
}

impl OptionName for Linecap {
    fn name(&self) -> String {
        match self {
            Linecap::Off => t!("tetris.options.linecap.off"),
            Linecap::KillScreenX2 => t!("tetris.options.linecap.killscreenx2"),
            Linecap::Halt => t!("tetris.options.linecap.halt"),
        }
        .into()
    }
}

impl OptionName for Gravity {
    fn name(&self) -> String {
        match self {
            Gravity::Level => t!("tetris.options.gravity.level"),
            Gravity::Locked => t!("tetris.options.gravity.locked"),
        }
        .into()
    }
}

impl OptionName for Seeding {
    fn name(&self) -> String {
        match self {
            Seeding::System => t!("tetris.options.seeding.system"),
            Seeding::Custom => t!("tetris.options.seeding.custom"),
        }
        .into()
    }
}

impl OptionName for Random {
    fn name(&self) -> String {
        match self {
            Random::Uniform => t!("tetris.options.random.uniform"),
            Random::Classic => t!("tetris.options.random.classic"),
            Random::Modern => t!("tetris.options.random.modern"),
        }
        .into()
    }
}

impl OptionName for ScoreDisplay {
    fn name(&self) -> String {
        match self {
            ScoreDisplay::Decimal => t!("tetris.options.score_display.decimal"),
            ScoreDisplay::Classic => t!("tetris.options.score_display.classic"),
            ScoreDisplay::Base36 => t!("tetris.options.score_display.base36"),
        }
        .into()
    }
}

impl OptionName for LevelDisplay {
    fn name(&self) -> String {
        match self {
            LevelDisplay::Decimal => t!("tetris.options.level_display.decimal"),
            LevelDisplay::Classic => t!("tetris.options.level_display.classic"),
        }
        .into()
    }
}

impl OptionName for TVSystem {
    fn name(&self) -> String {
        match self {
            TVSystem::NTSC => "NTSC",
            TVSystem::PAL => "PAL",
        }
        .into()
    }
}

impl OptionName for NextPieceHint {
    fn name(&self) -> String {
        match self {
            NextPieceHint::Off => t!("tetris.options.next_piece_hint.off"),
            NextPieceHint::Classic => t!("tetris.options.next_piece_hint.classic"),
            NextPieceHint::Modern => t!("tetris.options.next_piece_hint.modern"),
        }
        .into()
    }
}

impl OptionName for Invisible {
    fn name(&self) -> String {
        match self {
            Invisible::Off => t!("tetris.options.invisible.off"),
            Invisible::On => t!("tetris.options.invisible.on"),
        }
        .into()
    }
}

impl OptionName for TetrisFlash {
    fn name(&self) -> String {
        match self {
            TetrisFlash::On => t!("tetris.options.tetris_flash.on"),
            TetrisFlash::Off => t!("tetris.options.tetris_flash.off"),
        }
        .into()
    }
}

#[cfg(all(not(target_arch = "wasm32"), feature = "fps_limiter"))]
impl OptionName for FPSLimiter {
    fn name(&self) -> String {
        match self {
            FPSLimiter::Unlimited => t!("tetris.options.fps_limiter.unlimited"),
            FPSLimiter::F240 => t!("tetris.options.fps_limiter.240fps"),
            FPSLimiter::F480 => t!("tetris.options.fps_limiter.480fps"),
        }
        .into()
    }
}

impl OptionName for ShowFPS {
    fn name(&self) -> String {
        match self {
            ShowFPS::Off => t!("tetris.options.show_fps.off"),
            ShowFPS::On => t!("tetris.options.show_fps.on"),
        }
        .into()
    }
}

impl OptionName for ControllerMapping {
    fn name(&self) -> String {
        match self {
            ControllerMapping::MappingA => t!("tetris.options.controller_mapping.mapping_a"),
            ControllerMapping::MappingB => t!("tetris.options.controller_mapping.mapping_b"),
        }
        .into()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl OptionName for WindowMode {
    fn name(&self) -> String {
        match self {
            WindowMode::Windowed => t!("tetris.options.window_mode.windowed"),
            WindowMode::BorderlessFullscreen => {
                t!("tetris.options.window_mode.borderless_fullscreen")
            }
        }
        .into()
    }
}

impl OptionName for ScaleFactor {
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
