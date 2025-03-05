use bevy::{
    color::palettes::css::GREEN,
    prelude::*,
    window::{CursorOptions, PresentMode, WindowResolution},
};

#[macro_use]
extern crate num_derive;
extern crate num_traits;

mod app_state;
mod audio;
mod enum_iter;
mod game;
mod game_option_menu;
mod init;
mod input;
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
                        position: WindowPosition::Automatic,
                        cursor_options: CursorOptions {
                            visible: false,
                            ..default()
                        },
                        fit_canvas_to_parent: true,
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
                ..default()
            },
        })
        .insert_resource(ClearColor(Color::BLACK)) // application background color
        .init_state::<AppState>()
        .add_systems(Startup, setup_camera)
        .add_plugins((
            input::plugin::setup,
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
    commands.spawn(Camera2d::default());
}
