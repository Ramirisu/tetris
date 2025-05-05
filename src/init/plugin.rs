use bevy::prelude::*;

use crate::app_state::AppState;

pub fn setup(app: &mut App) {
    app.add_systems(Update, (init_system).run_if(in_state(AppState::Init)));
}

fn init_system(mut app_state: ResMut<NextState<AppState>>) {
    app_state.set(AppState::LanguageMenu);
}
