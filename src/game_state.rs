use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Menu,
    Game,
}
