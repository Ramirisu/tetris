use bevy::prelude::*;

use super::{
    board::Board,
    render::RenderConfig,
    tick::{EntryDelayTick, FallTick, LineClearTick},
    timer::{DelayAutoShiftTimer, GameTimer, PressDownTimer},
    transition::Transition,
};

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum PlayerState {
    #[default]
    GameRunning,
    GameLineClear,
    GameUpdateSquareImageAssets,
    GameEntryDelay,
    GamePause,
    GameOver,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct PlayerConfig {
    pub start_level: usize,
    pub transition: Transition,
    pub lv39_linecap: bool,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        Self {
            start_level: 0,
            lv39_linecap: false,
            transition: Transition::Classic,
        }
    }
}

#[derive(Resource)]
pub struct PlayerData {
    pub rc: RenderConfig,
    pub board: Board,
    pub game_timer: GameTimer,
    pub lock_curr_piece_immediately: bool,
    pub can_press_down: bool,
    pub press_down_timer: PressDownTimer,
    pub das_timer: DelayAutoShiftTimer,
    pub fall_tick: FallTick,
    pub line_clear_tick: LineClearTick,
    pub line_clear_rows: Vec<usize>,
    pub line_clear_phase: LineClearPhase,
    pub entry_delay_tick: EntryDelayTick,
}

impl PlayerData {
    pub fn new(config: PlayerConfig) -> Self {
        Self {
            rc: RenderConfig::default(),
            board: Board::new(config.start_level, config.transition),
            game_timer: GameTimer::default(),
            lock_curr_piece_immediately: false,
            can_press_down: false,
            press_down_timer: PressDownTimer::default(),
            das_timer: DelayAutoShiftTimer::default(),
            fall_tick: FallTick::new(config.start_level, config.lv39_linecap),
            line_clear_tick: LineClearTick::default(),
            line_clear_rows: default(),
            line_clear_phase: LineClearPhase::default(),
            entry_delay_tick: EntryDelayTick::default(),
        }
    }
}

impl Default for PlayerData {
    fn default() -> Self {
        Self::new(PlayerConfig::default())
    }
}

pub struct LineClearPhase {
    cols: Option<(usize, usize)>, // (left, right)
}

impl LineClearPhase {
    pub fn new() -> Self {
        const COLS: usize = Board::BOARD_COLS;
        Self {
            cols: if COLS % 2 == 0 {
                Some((COLS / 2 - 1, COLS / 2))
            } else {
                Some((COLS / 2, COLS / 2))
            },
        }
    }

    pub fn next_cols(&mut self) -> Option<(usize, usize)> {
        self.cols.map(|cols| {
            if cols.0 > 0 {
                self.cols = Some((cols.0 - 1, cols.1 + 1));
            } else {
                self.cols = None
            }
            cols
        })
    }
}

impl Default for LineClearPhase {
    fn default() -> Self {
        Self::new()
    }
}
