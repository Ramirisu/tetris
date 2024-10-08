use bevy::prelude::*;

#[derive(Clone, Copy, Resource)]
pub struct SplashTransform {
    scale: f32,
}

impl SplashTransform {
    pub fn new(scale: f32) -> Self {
        Self { scale }
    }

    pub fn fs_medium(&self) -> f32 {
        self.scale * 36.0
    }

    pub fn fs_large(&self) -> f32 {
        self.scale * 48.0
    }
}

impl Default for SplashTransform {
    fn default() -> Self {
        Self::new(1.0)
    }
}
