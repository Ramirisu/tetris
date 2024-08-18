use bevy::{
    color::palettes::css::GREEN,
    prelude::*,
    window::{PresentMode, WindowMode, WindowResolution},
};

mod app_state;
mod controller;
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
                resolution: WindowResolution::new(1280.0, 960.0),
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
        .insert_resource(ClearColor(Color::BLACK)) // application background color
        .init_state::<AppState>()
        .add_systems(Startup, setup_camera)
        .add_systems(Update, global_handle_input_system)
        .add_plugins((
            controller::setup,
            splash::plugin::setup,
            menu::plugin::setup,
            game::plugin::setup,
        ))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn global_handle_input_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    mut window: Query<&mut Window>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        app_exit_events.send(bevy::app::AppExit::Success);
    }
    if keys.just_pressed(KeyCode::F11) {
        let mut window = window.single_mut();
        match window.mode {
            WindowMode::Windowed => {
                window.mode = WindowMode::Fullscreen;
                window.present_mode = PresentMode::AutoNoVsync;
            }
            WindowMode::Fullscreen => {
                window.mode = WindowMode::Windowed;
                window.present_mode = PresentMode::AutoVsync;
            }
            _ => (),
        }
    }
}
