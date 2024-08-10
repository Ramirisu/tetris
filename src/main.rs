use bevy::prelude::*;

mod game;
mod game_state;
mod menu;
mod utility;

use game_state::GameState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins((menu::plugin::setup, game::plugin::setup))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
