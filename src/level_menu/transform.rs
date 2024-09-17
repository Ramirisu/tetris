use bevy::prelude::*;

#[derive(Clone, Copy, Resource)]
pub struct LevelMenuTransform {
    scale: f32,
}

impl LevelMenuTransform {
    pub fn new(scale: f32) -> Self {
        Self { scale }
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }
}

impl Default for LevelMenuTransform {
    fn default() -> Self {
        Self::new(1.0)
    }
}
