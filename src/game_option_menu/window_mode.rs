use std::fmt::Display;

use num_traits::FromPrimitive;

#[derive(Default, Clone, Copy, FromPrimitive)]
pub enum WindowMode {
    #[default]
    Windowed,
    BorderlessFullscreen,
    Fullscreen,
}

impl WindowMode {
    pub fn enum_has_prev(&self) -> bool {
        <Self as FromPrimitive>::from_i64(*self as i64 - 1).is_some()
    }

    pub fn enum_has_next(&self) -> bool {
        <Self as FromPrimitive>::from_i64(*self as i64 + 1).is_some()
    }

    pub fn enum_prev(&mut self) -> bool {
        match FromPrimitive::from_i64(*self as i64 - 1) {
            Some(n) => {
                *self = n;
                true
            }
            None => false,
        }
    }

    pub fn enum_next(&mut self) -> bool {
        match FromPrimitive::from_i64(*self as i64 + 1) {
            Some(n) => {
                *self = n;
                true
            }
            None => false,
        }
    }

    pub fn get_window_mode(&self) -> bevy::window::WindowMode {
        match self {
            WindowMode::Windowed => bevy::window::WindowMode::Windowed,
            WindowMode::BorderlessFullscreen => bevy::window::WindowMode::BorderlessFullscreen,
            WindowMode::Fullscreen => bevy::window::WindowMode::Fullscreen,
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
