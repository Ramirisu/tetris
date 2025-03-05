use bevy::{color::palettes::css::WHITE, prelude::*};

use crate::{
    app_state::AppState,
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
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            SplashEntityMarker,
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    display: Display::Grid,
                    grid_template_columns: vec![GridTrack::auto(); TETRIS_BITMAP[0].len()],
                    margin: UiRect::all(Val::Px(transform.fs_medium())),
                    ..default()
                })
                .with_children(|parent| {
                    TETRIS_BITMAP.iter().for_each(|rows| {
                        rows.iter().for_each(|sqr| {
                            parent.spawn((
                                Node {
                                    width: Val::Px(transform.fs_medium()),
                                    height: Val::Px(transform.fs_medium()),
                                    ..default()
                                },
                                ImageNode::new(logo_images[(*sqr) as usize].clone()),
                            ));
                        })
                    });
                });

            parent
                .spawn(Node {
                    margin: UiRect::all(Val::Px(transform.fs_large())),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("PRESS START"),
                        TextFont::from_font_size(transform.fs_medium()),
                        TextColor::from(WHITE),
                    ));
                });
        });
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
        app_state.set(AppState::GameModeMenu);
    }
}
