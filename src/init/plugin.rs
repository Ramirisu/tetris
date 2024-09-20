use bevy::prelude::*;

use crate::app_state::AppState;

pub fn setup(app: &mut App) {
    app.add_systems(
        Update,
        enter_splash_system.run_if(in_state(AppState::Startup)),
    );
}

fn enter_splash_system(mut app_state: ResMut<NextState<AppState>>) {
    app_state.set(AppState::Splash);
}
