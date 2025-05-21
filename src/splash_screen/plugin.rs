use bevy::{color::palettes::css::WHITE, prelude::*};

use crate::{
    app_state::AppState,
    input::{controller_mapping::ControllerMapping, player_inputs::PlayerInputs},
    logo::logo,
    utility::{effect::flicker, entity::despawn_all},
};

pub fn setup(app: &mut App) {
    app.add_systems(OnEnter(AppState::SplashScreen), setup_screen)
        .add_systems(
            Update,
            (handle_input_system, update_ui_system).run_if(in_state(AppState::SplashScreen)),
        )
        .add_systems(
            OnExit(AppState::SplashScreen),
            despawn_all::<SplashScreenEntityMarker>,
        );
}

#[derive(Component)]
struct SplashScreenEntityMarker;

#[derive(Component)]
struct PressStartEntityMarker;

fn setup_screen(mut commands: Commands, mut image_assets: ResMut<Assets<Image>>) {
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
        SplashScreenEntityMarker,
        Children::spawn((
            Spawn((
                Node {
                    margin: UiRect::all(Val::Px(40.0)),
                    ..default()
                },
                Children::spawn(Spawn(logo(Val::Px(40.0), &mut image_assets))),
            )),
            Spawn((
                Node {
                    margin: UiRect::all(Val::Px(40.0)),
                    ..default()
                },
                Children::spawn(Spawn((
                    Text::new(t!("tetris.splash.press_start")),
                    TextFont::from_font_size(40.0),
                    TextColor::from(WHITE),
                    PressStartEntityMarker,
                ))),
            )),
        )),
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
        app_state.set(AppState::LanguageMenu);
    }
}

fn update_ui_system(
    time: Res<Time>,
    mut query: Query<&mut TextColor, With<PressStartEntityMarker>>,
) {
    if let Ok(mut color) = query.single_mut() {
        color.set_alpha(flicker(time.elapsed_secs(), 2.0));
    }
}
