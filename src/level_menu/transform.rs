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

    pub fn fs_small(&self) -> f32 {
        self.scale * 24.0
    }

    pub fn fs_medium(&self) -> f32 {
        self.scale * 36.0
    }

    pub fn fs_large(&self) -> f32 {
        self.scale * 48.0
    }
}

impl Default for LevelMenuTransform {
    fn default() -> Self {
        Self::new(1.0)
    }
}
