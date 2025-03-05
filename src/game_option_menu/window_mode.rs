use std::fmt::Display;

use crate::enum_iter;

#[derive(Default, Clone, Copy, FromPrimitive)]
pub enum WindowMode {
    #[default]
    Windowed,
    BorderlessFullscreen,
    Fullscreen,
}

enum_iter::enum_iter_derive!(WindowMode);

impl Into<bevy::window::WindowMode> for WindowMode {
    fn into(self) -> bevy::window::WindowMode {
        match self {
            WindowMode::Windowed => bevy::window::WindowMode::Windowed,
            WindowMode::BorderlessFullscreen => bevy::window::WindowMode::BorderlessFullscreen(
                bevy::window::MonitorSelection::Current,
            ),
            WindowMode::Fullscreen => {
                bevy::window::WindowMode::Fullscreen(bevy::window::MonitorSelection::Current)
            }
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
