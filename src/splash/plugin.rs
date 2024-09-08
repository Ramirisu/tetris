use bevy::{color::palettes::css::WHITE, prelude::*};

use crate::{
    app_state::AppState,
    inputs::PlayerInputs,
    logo::{load_logo_images, TETRIS_BITMAP},
    utility::despawn_all,
};

pub fn setup(app: &mut App) {
    app.add_systems(OnEnter(AppState::Splash), setup_screen)
        .add_systems(
            Update,
            handle_input_system.run_if(in_state(AppState::Splash)),
        )
        .add_systems(OnExit(AppState::Splash), despawn_all::<SplashEntityMarker>);
}

#[derive(Component)]
struct SplashEntityMarker;

fn setup_screen(mut commands: Commands, mut image_assets: ResMut<Assets<Image>>) {
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
            parent.spawn(
                TextBundle::from_section(
                    "Classic",
                    TextStyle {
                        font_size: 40.0,
                        color: WHITE.into(),
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(40.0)),
                    ..default()
                }),
            );
            parent
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        grid_template_columns: vec![GridTrack::auto(); TETRIS_BITMAP[0].len()],
                        margin: UiRect::all(Val::Px(40.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    TETRIS_BITMAP.iter().for_each(|rows| {
                        rows.iter().for_each(|square| {
                            parent.spawn((
                                NodeBundle {
                                    style: Style {
                                        width: Val::Px(36.0),
                                        height: Val::Px(36.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                                UiImage {
                                    texture: logo_images[(*square) as usize].clone(),
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
                        font_size: 30.0,
                        color: WHITE.into(),
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(60.0)),
                    ..default()
                }),
            );
        });
}

fn handle_input_system(
    player_inputs: Res<PlayerInputs>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    if player_inputs.start {
        app_state.set(AppState::GameModeMenu);
    }
}
