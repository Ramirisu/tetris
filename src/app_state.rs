use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Init,
    LoadingScreen,
    SplashScreen,
    LanguageScreen,
    GameOptionsScreen,
    GameLevelsScreen,
    GameScreen,
}
