use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Init,
    LoadingScreen,
    SplashScreen,
    LanguageMenu,
    SettingsMenu,
    LevelMenu,
    Game,
}
