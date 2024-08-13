use bevy::{color::palettes::css::GREEN, prelude::*, window::WindowResolution};

mod app_state;
mod game;
mod menu;
mod splash;
mod utility;

use app_state::AppState;
use bevy_dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1280., 960.),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextStyle {
                    font_size: 40.0,
                    color: GREEN.into(),
                    font: default(),
                },
            },
        })
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .add_plugins((
            splash::plugin::setup,
            menu::plugin::setup,
            game::plugin::setup,
        ))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
