use bevy::prelude::*;

#[derive(Clone, Copy, Resource)]
pub struct SplashTransform {
    scale: f32,
}

impl SplashTransform {
    pub fn new(scale: f32) -> Self {
        Self { scale }
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }
}

impl Default for SplashTransform {
    fn default() -> Self {
        Self::new(1.0)
    }
}
