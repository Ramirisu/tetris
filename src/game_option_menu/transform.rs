use bevy::prelude::*;

#[derive(Clone, Copy, Resource)]
pub struct GameOptionMenuTransform {
    scale: f32,
}

impl GameOptionMenuTransform {
    pub fn new(scale: f32) -> Self {
        Self { scale }
    }

    pub fn fs_small(&self) -> f32 {
        self.scale * 24.0
    }

    pub fn fs_medium(&self) -> f32 {
        self.scale * 36.0
    }
}

impl Default for GameOptionMenuTransform {
    fn default() -> Self {
        Self::new(1.0)
    }
}
