use bevy::{prelude::*, time::Stopwatch};

use super::{
    board::Board,
    das_indicator::DASIndicator,
    game::GameConfig,
    next_piece_hint::NextPieceHint,
    tick::{EntryDelayTick, FallTick, LineClearTick},
    timer::{DelayAutoShiftTimer, GameTimer, PressDownTimer},
};

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum PlayerPhase {
    #[default]
    Dropping,
    LineClear,
    EntryDelay,
    Over,
}

#[derive(Resource)]
pub struct PlayerData {
    pub next_piece_hint: NextPieceHint,
    pub das_indicator: DASIndicator,
    pub board: Board,
    pub stopwatch: Stopwatch,
    pub game_timer: GameTimer,
    pub lock_curr_piece_immediately: bool,
    pub can_press_down: bool,
    pub press_down_timer: PressDownTimer,
    pub das_timer: DelayAutoShiftTimer,
    pub fall_tick: FallTick,
    pub line_clear_rows: Vec<usize>,
    pub line_clear_phase: LineClearPhase,
    pub entry_delay_tick: EntryDelayTick,
}

impl PlayerData {
    pub fn new(config: GameConfig) -> Self {
        Self {
            next_piece_hint: config.next_piece_hint,
            das_indicator: config.das_indicator,
            board: Board::new(config.start_level, config.transition),
            stopwatch: Stopwatch::new(),
            game_timer: GameTimer::default(),
            lock_curr_piece_immediately: false,
            can_press_down: false,
            press_down_timer: PressDownTimer::default(),
            das_timer: DelayAutoShiftTimer::default(),
            fall_tick: FallTick::new(config.start_level, config.linecap, config.drop_speed),
            line_clear_rows: default(),
            line_clear_phase: LineClearPhase::default(),
            entry_delay_tick: EntryDelayTick::default(),
        }
    }
}

impl Default for PlayerData {
    fn default() -> Self {
        Self::new(GameConfig::default())
    }
}

pub struct LineClearPhase {
    cols: usize,
    total_phases: usize,
    curr_phase: usize,
    pub tick: LineClearTick,
}

impl LineClearPhase {
    pub fn new() -> Self {
        let cols = Board::BOARD_COLS;
        let total_phases = (cols + 1) / 2;
        Self {
            cols,
            total_phases,
            curr_phase: 0,
            tick: LineClearTick::new(total_phases),
        }
    }

    pub fn next(&mut self) -> Option<(usize, usize, bool)> {
        if self.curr_phase < self.total_phases {
            self.curr_phase += 1;
            let left = self.total_phases - self.curr_phase;
            let right = self.cols - left - 1;
            Some((left, right, self.curr_phase == self.total_phases))
        } else {
            None
        }
    }
}

impl Default for LineClearPhase {
    fn default() -> Self {
        Self::new()
    }
}
