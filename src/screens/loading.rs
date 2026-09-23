use std::time::Duration;

use bevy::prelude::*;

use crate::{
    app_state::AppState,
    input::{controller_mapping::ControllerMapping, player_inputs::PlayerInputs},
    utility::entity::despawn_all,
};

pub fn plugin(app: &mut App) {
    app.insert_resource(LoadingScreenIconTimeDuration::default())
        .add_systems(OnEnter(AppState::LoadingScreen), setup_screen)
        .add_systems(
            Update,
            (handle_input_system, update_ui_system).run_if(in_state(AppState::LoadingScreen)),
        )
        .add_systems(
            OnExit(AppState::LoadingScreen),
            despawn_all::<LoadingScreenEntityMarker>,
        );
}

#[derive(Component)]
struct LoadingScreenEntityMarker;

#[derive(Component)]
struct LoadingScreenIconEntityMarker;

#[derive(Default, Resource)]
struct LoadingScreenIconTimeDuration {
    elapsed: Duration,
}

fn setup_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        LoadingScreenEntityMarker,
        Children::spawn(Spawn((
            Node {
                margin: UiRect::all(Val::Px(40.0)),
                ..default()
            },
            ImageNode::new(asset_server.load("images/bevy_logo_dark.png"))
                .with_color(Srgba::new(1.0, 1.0, 1.0, 0.0).into()),
            LoadingScreenIconEntityMarker,
        ))),
    ));
}

fn handle_input_system(
    keys: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    controller_mapping: Res<ControllerMapping>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    let player_inputs = PlayerInputs::with_keyboard(&keys)
        | PlayerInputs::with_gamepads(gamepads, *controller_mapping);

    if player_inputs.start.just_pressed {
        app_state.set(AppState::SplashScreen);
    }
}

fn update_ui_system(
    t: Res<Time>,
    mut duration: ResMut<LoadingScreenIconTimeDuration>,
    mut q: Query<&mut ImageNode, With<LoadingScreenIconEntityMarker>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    duration.elapsed += t.delta();

    if let Ok(mut img) = q.single_mut() {
        if let Some(alpha) = loading_icon_alpha(duration.elapsed.as_secs_f32()) {
            img.color.set_alpha(alpha);
        } else {
            app_state.set(AppState::SplashScreen);
        }
    }
}

fn loading_icon_alpha(seconds: f32) -> Option<f32> {
    match seconds {
        0.0..2.0 => Some(0.0),
        2.0..4.0 => Some((seconds - 2.0) / 2.0),
        4.0..6.0 => Some(1.0),
        6.0..8.0 => Some((8.0 - seconds) / 2.0),
        8.0..9.0 => Some(0.0),
        _ => None,
    }
}
