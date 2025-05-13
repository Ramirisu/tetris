use bevy::prelude::*;

use super::{
    gravity::Gravity, invisible::Invisible, linecap::Linecap, next_piece_hint::NextPieceHint,
    scoring::Scoring, seed::Seed, seeding::Seeding, transition::Transition, tv_system::TVSystem,
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
    pub linecap_level: usize,
    pub gravity: Gravity,
    pub seeding: Seeding,
    pub seed: Seed,
    pub scoring: Scoring,
    pub tv_system: TVSystem,
    pub next_piece_hint: NextPieceHint,
    pub invisible: Invisible,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            start_level: 0,
            transition: Transition::default(),
            linecap: Linecap::default(),
            linecap_level: 39,
            gravity: Gravity::default(),
            seeding: Seeding::default(),
            seed: Seed::default(),
            scoring: Scoring::default(),
            tv_system: TVSystem::default(),
            next_piece_hint: NextPieceHint::default(),
            invisible: Invisible::default(),
        }
    }
}
