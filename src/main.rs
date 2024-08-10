use bevy::prelude::*;

mod app_state;
mod game;
mod menu;
mod utility;

use app_state::AppState;
use bevy_dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextStyle {
                    font_size: 40.0,
                    color: Color::srgb(0.0, 1.0, 0.0),
                    font: default(),
                },
            },
        })
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .add_plugins((menu::plugin::setup, game::plugin::setup))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
