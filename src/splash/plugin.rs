use bevy::{color::palettes::css::WHITE, prelude::*};

use crate::{
    app_state::AppState,
    controller::Controller,
    input::{controller_mapping::ControllerMapping, player_inputs::PlayerInputs},
    logo::{load_logo_images, TETRIS_BITMAP},
    utility::despawn_all,
};

use super::transform::SplashTransform;

pub fn setup(app: &mut App) {
    app.insert_resource(SplashTransform::default())
        .add_systems(OnEnter(AppState::Splash), setup_screen)
        .add_systems(
            Update,
            handle_input_system.run_if(in_state(AppState::Splash)),
        )
        .add_systems(OnExit(AppState::Splash), despawn_all::<SplashEntityMarker>);
}

#[derive(Component)]
struct SplashEntityMarker;

fn setup_screen(
    mut commands: Commands,
    mut image_assets: ResMut<Assets<Image>>,
    transform: Res<SplashTransform>,
) {
    let logo_images = load_logo_images(&mut image_assets);

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            SplashEntityMarker,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        grid_template_columns: vec![GridTrack::auto(); TETRIS_BITMAP[0].len()],
                        margin: UiRect::all(Val::Px(transform.fs_medium())),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    TETRIS_BITMAP.iter().for_each(|rows| {
                        rows.iter().for_each(|sqr| {
                            parent.spawn((
                                NodeBundle {
                                    style: Style {
                                        width: Val::Px(transform.fs_medium()),
                                        height: Val::Px(transform.fs_medium()),
                                        ..default()
                                    },
                                    ..default()
                                },
                                UiImage {
                                    texture: logo_images[(*sqr) as usize].clone(),
                                    ..default()
                                },
                            ));
                        })
                    });
                });
            parent.spawn(
                TextBundle::from_section(
                    "PRESS START",
                    TextStyle {
                        font_size: transform.fs_medium(),
                        color: WHITE.into(),
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(transform.fs_large())),
                    ..default()
                }),
            );
        });
}

fn handle_input_system(
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<GamepadButton>>,
    controller: Res<Controller>,
    controller_mapping: Res<ControllerMapping>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    let player_inputs = PlayerInputs::with_keyboard(&keys)
        | PlayerInputs::with_gamepads(&buttons, &controller, *controller_mapping);

    if player_inputs.start.just_pressed {
        app_state.set(AppState::GameModeMenu);
    }
}
