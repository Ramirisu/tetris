use bevy::prelude::*;

use super::{
    gravity::Gravity, invisible::Invisible, level::Level, level_display::LevelDisplay,
    linecap::Linecap, next_piece_hint::NextPieceHint, random::Random, score_display::ScoreDisplay,
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
    pub start_level: Level,
    pub transition: Transition,
    pub scoring: Scoring,
    pub linecap: Linecap,
    pub linecap_level: Level,
    pub gravity: Gravity,
    pub seeding: Seeding,
    pub seed: Seed,
    pub random: Random,
    pub score_display: ScoreDisplay,
    pub level_display: LevelDisplay,
    pub tv_system: TVSystem,
    pub next_piece_hint: NextPieceHint,
    pub invisible: Invisible,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            start_level: Level(0),
            transition: Transition::default(),
            scoring: Scoring::default(),
            linecap: Linecap::default(),
            linecap_level: Level(39),
            gravity: Gravity::default(),
            seeding: Seeding::default(),
            seed: Seed::default(),
            random: Random::default(),
            score_display: ScoreDisplay::default(),
            level_display: LevelDisplay::default(),
            tv_system: TVSystem::default(),
            next_piece_hint: NextPieceHint::default(),
            invisible: Invisible::default(),
        }
    }
}
