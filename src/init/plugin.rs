use bevy::prelude::*;
use strum::IntoEnumIterator;

use crate::{
    app_state::AppState,
    language_menu::plugin::{Language, LanguageMenuData},
};

pub fn setup(app: &mut App) {
    app.add_systems(
        Update,
        (init_locale_system, complete_initialization_system)
            .chain()
            .run_if(in_state(AppState::Init)),
    );
}

fn init_locale_system(mut lang_menu_data: ResMut<LanguageMenuData>) {
    if let Some(locale) = sys_locale::get_locale() {
        info!("System locale: {}", locale);
        rust_i18n::set_locale(&locale);
        for lang in Language::iter() {
            if lang.locale() == locale {
                lang_menu_data.language_selection = lang;
            }
        }
    } else {
        warn!("Unable to get system locale");
    }
}

fn complete_initialization_system(mut app_state: ResMut<NextState<AppState>>) {
    app_state.set(AppState::LoadingScreen);
}
