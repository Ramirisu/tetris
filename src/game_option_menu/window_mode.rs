use std::fmt::Display;

use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, FromRepr};

use crate::enum_advance;

#[derive(Default, Clone, Copy, FromRepr, EnumIter, EnumCount)]
pub enum WindowMode {
    #[default]
    Windowed,
    BorderlessFullscreen,
    Fullscreen,
}

enum_advance::enum_advance_derive!(WindowMode);

impl Into<bevy::window::WindowMode> for WindowMode {
    fn into(self) -> bevy::window::WindowMode {
        match self {
            WindowMode::Windowed => bevy::window::WindowMode::Windowed,
            WindowMode::BorderlessFullscreen => bevy::window::WindowMode::BorderlessFullscreen(
                bevy::window::MonitorSelection::Current,
            ),
            WindowMode::Fullscreen => bevy::window::WindowMode::Fullscreen(
                bevy::window::MonitorSelection::Current,
                bevy::window::VideoModeSelection::Current,
            ),
        }
    }
}

impl Display for WindowMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WindowMode::Windowed => f.write_str("WINDOWED"),
            WindowMode::BorderlessFullscreen => f.write_str("BORDERLESS"),
            WindowMode::Fullscreen => f.write_str("FULLSCREEN"),
        }
    }
}
