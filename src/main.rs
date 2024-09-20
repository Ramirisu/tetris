use bevy::{
    color::palettes::css::GREEN,
    prelude::*,
    window::{Cursor, PresentMode, WindowResolution},
};

#[macro_use]
extern crate num_derive;
extern crate num_traits;

mod app_state;
mod audio;
mod controller;
mod game;
mod game_option_menu;
mod init;
mod inputs;
mod level_menu;
mod logo;
mod scale;
mod splash;
mod utility;

use app_state::AppState;
use bevy_dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(960.0, 720.0)
                            .with_scale_factor_override(1.0),
                        resize_constraints: WindowResizeConstraints {
                            min_width: 960.0,
                            min_height: 720.0,
                            max_width: f32::INFINITY,
                            max_height: f32::INFINITY,
                        },
                        present_mode: PresentMode::AutoNoVsync,
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        cursor: Cursor {
                            visible: false,
                            ..default()
                        },
                        title: "TETRIS".into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextStyle {
                    font_size: 20.0,
                    color: GREEN.into(),
                    font: default(),
                },
            },
        })
        .insert_resource(ClearColor(Color::BLACK)) // application background color
        .init_state::<AppState>()
        .add_systems(Startup, setup_camera)
        .add_plugins((
            controller::setup,
            inputs::setup,
            scale::plugin::setup,
            audio::plugin::setup,
            init::plugin::setup,
            splash::plugin::setup,
            game_option_menu::plugin::setup,
            level_menu::plugin::setup,
            game::plugin::setup,
        ))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
