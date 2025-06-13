#![windows_subsystem = "windows"]

use bevy::{
    asset::load_internal_binary_asset,
    color::palettes::css::GREEN,
    prelude::*,
    window::{EnabledButtons, PresentMode, WindowResolution},
};

mod app_state;
mod audio;
mod game_screen;
mod init;
mod input;
mod language_menu;
mod level_menu;
mod loading_screen;
mod logo;
mod settings_menu;
mod splash_screen;
mod utility;

use app_state::AppState;
use bevy_dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use settings_menu::{
    scale_factor::{WINDOW_HEIGHT, WINDOW_WIDTH},
    show_fps::ShowFPS,
};

#[macro_use]
extern crate rust_i18n;

i18n!("locales", fallback = "en");

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT)
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
    .add_systems(Startup, setup_camera)
    .add_plugins((
        input::plugin::setup,
        audio::plugin::setup,
        init::plugin::setup,
        loading_screen::plugin::setup,
        language_menu::plugin::setup,
        splash_screen::plugin::setup,
        settings_menu::plugin::setup,
        level_menu::plugin::setup,
        game_screen::plugin::setup,
    ));

    load_internal_binary_asset!(
        app,
        TextFont::default().font,
        "../assets/fonts/NotoSansCJK-Regular.ttc",
        |bytes: &[u8], _path: String| { Font::try_from_bytes(bytes.to_vec()).unwrap() }
    );

    app.run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
