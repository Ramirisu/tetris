// Do not pop up a terminal window for the release build, we only need it to show logs in debug build.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{
    color::palettes::css::GREEN,
    prelude::*,
    window::{EnabledButtons, PresentMode, WindowResolution},
};

mod app_state;
mod audio;
mod game;
mod init;
mod input;
mod logo;
mod options;
mod screens;
mod utility;

use app_state::AppState;
use bevy_dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use options::{
    scale_factor::{WINDOW_HEIGHT, WINDOW_WIDTH},
    show_fps::ShowFPS,
};

use crate::input::controller_mapping::ControllerMapping;

#[macro_use]
extern crate rust_i18n;

i18n!("locales", fallback = "en");

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(
                            WINDOW_WIDTH as u32,
                            WINDOW_HEIGHT as u32,
                        )
                        .with_scale_factor_override(1.0),
                        present_mode: PresentMode::AutoNoVsync,
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resizable: false,
                        enabled_buttons: EnabledButtons {
                            minimize: true,
                            maximize: false,
                            close: true,
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
                text_color: GREEN.into(),
                enabled: ShowFPS::default().is_enabled(),
                ..default()
            },
        })
        .insert_resource(ClearColor(Color::BLACK)) // application background color
        .init_state::<AppState>()
        .insert_resource(ControllerMapping::default())
        .add_systems(Startup, setup_camera)
        .add_plugins((
            audio::plugin,
            init::plugin,
            screens::loading::plugin,
            screens::splash::plugin,
            screens::language::plugin,
            screens::game_options::plugin,
            screens::game_levels::plugin,
            screens::game::plugin,
        ))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
