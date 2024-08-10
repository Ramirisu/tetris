use bevy::prelude::*;

use crate::{game_state::GameState, utility::clear_screen};

pub fn setup(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), setup_screen)
        .add_systems(OnExit(GameState::Game), clear_screen::<OnGameScreen>);
}

#[derive(Component)]
struct OnGameScreen;

fn setup_screen(mut _commands: Commands) {}
