use bevy::{prelude::*, time::Stopwatch};

use super::{
    board::Board,
    game::GameConfig,
    timer::{DelayAutoShiftTimer, EntryDelayTimer, LineClearTimer, PressDownTimer, SoftDropTimer},
    tv_system::TVSystem,
};

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum PlayerPhase {
    #[default]
    Init,
    Dropping,
    LineClear,
    EntryDelay,
    Over,
}

#[derive(Resource)]
pub struct PlayerData {
    pub board: Board,
    pub stopwatch: Stopwatch,
    pub soft_drop_timer: SoftDropTimer,
    pub lock_curr_piece_immediately: bool,
    pub can_press_down: bool,
    pub press_down_timer: PressDownTimer,
    pub das_timer: DelayAutoShiftTimer,
    pub line_clear_rows: Vec<usize>,
    pub line_clear_phase: LineClearPhase,
    pub entry_delay_timer: EntryDelayTimer,
}

impl PlayerData {
    pub fn new(config: GameConfig) -> Self {
        Self {
            board: Board::new(
                config.start_level,
                config.transition,
                config.random,
                config.seeding,
                config.seed,
                config.next_piece_hint,
            ),
            stopwatch: Stopwatch::new(),
            soft_drop_timer: SoftDropTimer::new(
                config.start_level,
                config.linecap,
                config.linecap_level,
                config.gravity,
                config.tv_system,
                true,
            ),
            lock_curr_piece_immediately: false,
            can_press_down: false,
            press_down_timer: PressDownTimer::new(config.tv_system),
            das_timer: DelayAutoShiftTimer::new(config.tv_system),
            line_clear_rows: default(),
            line_clear_phase: LineClearPhase::new(config.tv_system),
            entry_delay_timer: EntryDelayTimer::new(0, config.tv_system),
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
    phase: usize,
    curr: usize,
    pub timer: LineClearTimer,
}

impl LineClearPhase {
    pub fn new(tv_system: TVSystem) -> Self {
        let cols = Board::BOARD_COLS;
        let phase = (cols + 1) / 2;
        Self {
            cols,
            phase,
            curr: 0,
            timer: LineClearTimer::new(phase as u32, tv_system),
        }
    }

    pub fn next(&mut self) -> Option<(usize, usize, bool)> {
        if self.curr < self.phase {
            self.curr += 1;
            let left = self.phase - self.curr;
            let right = self.cols - left - 1;
            Some((left, right, self.curr == self.phase))
        } else {
            None
        }
    }
}
