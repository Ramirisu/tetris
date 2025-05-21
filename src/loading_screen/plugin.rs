use std::time::Duration;

use bevy::prelude::*;

use crate::{
    app_state::AppState,
    input::{controller_mapping::ControllerMapping, player_inputs::PlayerInputs},
    utility::entity::despawn_all,
};

pub fn setup(app: &mut App) {
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
struct LoadingScreenIconTimeDuration(Duration);

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
    time: Res<Time>,
    mut duration: ResMut<LoadingScreenIconTimeDuration>,
    mut query: Query<&mut ImageNode, With<LoadingScreenIconEntityMarker>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    duration.0 += time.delta();

    if let Ok(mut img) = query.single_mut() {
        let t = duration.0.as_secs_f32();
        match t {
            0.0..2.0 => img.color.set_alpha(0.0),
            2.0..4.0 => img.color.set_alpha((t - 2.0) / 2.0),
            4.0..6.0 => img.color.set_alpha(1.0),
            6.0..8.0 => img.color.set_alpha((8.0 - t) / 2.0),
            8.0..9.0 => img.color.set_alpha(0.0),
            _ => app_state.set(AppState::SplashScreen),
        }
    }
}
