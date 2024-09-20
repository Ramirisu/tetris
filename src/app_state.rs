use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Startup,
    Splash,
    GameModeMenu,
    LevelMenu,
    Game,
    ChangeScale,
}
