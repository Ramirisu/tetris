use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, FromRepr};

use crate::utility::enum_advance;

#[derive(Default, Clone, Copy, PartialEq, Eq, FromRepr, EnumIter, EnumCount)]
pub enum WindowMode {
    #[default]
    Windowed,
    BorderlessFullscreen,
}

enum_advance::enum_advance_derive!(WindowMode);

impl Into<bevy::window::WindowMode> for WindowMode {
    fn into(self) -> bevy::window::WindowMode {
        match self {
            WindowMode::Windowed => bevy::window::WindowMode::Windowed,
            WindowMode::BorderlessFullscreen => bevy::window::WindowMode::BorderlessFullscreen(
                bevy::window::MonitorSelection::Current,
            ),
        }
    }
}
