use bevy::prelude::*;

use super::{
    das_counter::DASCounter, drop_speed::DropSpeed, linecap::Linecap,
    next_piece_hint::NextPieceHint, transition::Transition, tv_system::TVSystem,
};

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Running,
    Pause,
    Over,
}

#[derive(Clone, Copy, Eq, PartialEq, Resource)]
pub struct GameConfig {
    pub start_level: usize,
    pub transition: Transition,
    pub linecap: Linecap,
    pub drop_speed: DropSpeed,
    pub tv_system: TVSystem,
    pub next_piece_hint: NextPieceHint,
    pub das_counter: DASCounter,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            start_level: 0,
            linecap: Linecap::default(),
            transition: Transition::default(),
            drop_speed: DropSpeed::default(),
            tv_system: TVSystem::default(),
            next_piece_hint: NextPieceHint::default(),
            das_counter: DASCounter::default(),
        }
    }
}
