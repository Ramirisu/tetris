use audio::plugin::PlaySoundEvent;
use bevy::{
    color::palettes::css::GREEN,
    prelude::*,
    window::{PresentMode, WindowResolution},
};

mod app_state;
mod audio;
mod controller;
mod game;
mod game_option_menu;
mod inputs;
mod level_menu;
mod logo;
mod splash;
mod utility;

use app_state::AppState;
use bevy_dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use controller::Controller;
use inputs::{ControllerType, PlayerInputs};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(1280.0, 960.0),
                        present_mode: PresentMode::AutoNoVsync,
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
        .add_systems(Update, handle_input_system)
        .add_plugins((
            controller::setup,
            inputs::setup,
            audio::plugin::setup,
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

fn handle_input_system(
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<GamepadButton>>,
    controller: Res<Controller>,
    controller_type: Res<ControllerType>,
    mut e_play_sound: EventWriter<PlaySoundEvent>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    let player_inputs = PlayerInputs::with_keyboard(&keys)
        | PlayerInputs::with_gamepads(&buttons, &controller, *controller_type);

    if player_inputs.soft_reset {
        e_play_sound.send(PlaySoundEvent::StartGame);
        app_state.set(AppState::Splash);
    }
}
