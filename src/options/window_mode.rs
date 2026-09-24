use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, FromRepr};

#[derive(Default, Clone, Copy, PartialEq, Eq, FromRepr, EnumIter, EnumCount, tetris_macros::EnumAdvance)]
pub enum WindowMode {
    #[default]
    Windowed,
    BorderlessFullscreen,
}


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
