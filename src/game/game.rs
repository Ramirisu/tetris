use super::{
    drop_speed::DropSpeed, linecap::Linecap, next_piece_hint::NextPieceHint, transition::Transition,
};

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct GameConfig {
    pub start_level: usize,
    pub transition: Transition,
    pub linecap: Linecap,
    pub drop_speed: DropSpeed,
    pub next_piece_hint: NextPieceHint,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            start_level: 0,
            linecap: Linecap::default(),
            transition: Transition::default(),
            drop_speed: DropSpeed::default(),
            next_piece_hint: NextPieceHint::default(),
        }
    }
}
