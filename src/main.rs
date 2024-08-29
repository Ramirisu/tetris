use bevy::{color::palettes::css::GREEN, prelude::*, window::WindowResolution};

#[cfg(not(target_arch = "wasm32"))]
use bevy::window::{PresentMode, WindowMode};

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
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(1280.0, 1000.0),
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
    mut app_state: ResMut<NextState<AppState>>,
    #[cfg(not(target_arch = "wasm32"))] mut window: Query<&mut Window>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        app_state.set(AppState::Splash);
    }

    #[cfg(not(target_arch = "wasm32"))]
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
