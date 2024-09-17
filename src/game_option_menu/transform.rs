use bevy::prelude::*;

#[derive(Clone, Copy, Resource)]
pub struct GameOptionMenuTransform {
    scale: f32,
}

impl GameOptionMenuTransform {
    pub fn new(scale: f32) -> Self {
        Self { scale }
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }
}

impl Default for GameOptionMenuTransform {
    fn default() -> Self {
        Self::new(1.0)
    }
}
