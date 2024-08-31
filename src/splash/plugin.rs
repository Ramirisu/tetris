use bevy::{color::palettes::css::WHITE, prelude::*};

use crate::{
    app_state::AppState,
    controller::Controller,
    game::{
        palette::{get_empty_square_image, get_square_image, SquareImageSize},
        piece::PieceShape,
    },
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

#[rustfmt::skip]
pub const TETRIS_BITMAP: &[[u8; 21]; 5] = &[
    [1, 1, 1, 0, 2, 2, 2, 0, 3, 3, 3, 0, 4, 4, 4, 0, 5, 0, 6, 6, 6],
    [0, 1, 0, 0, 2, 0, 0, 0, 0, 3, 0, 0, 4, 0, 4, 0, 5, 0, 6, 0, 0],
    [0, 1, 0, 0, 2, 2, 2, 0, 0, 3, 0, 0, 4, 4, 4, 0, 5, 0, 6, 6, 6],
    [0, 1, 0, 0, 2, 0, 0, 0, 0, 3, 0, 0, 4, 4, 0, 0, 5, 0, 0, 0, 6],
    [0, 1, 0, 0, 2, 2, 2, 0, 0, 3, 0, 0, 4, 0, 4, 0, 5, 0, 6, 6, 6],
];

fn setup_screen(mut commands: Commands, mut image_assets: ResMut<Assets<Image>>) {
    let square_images = [
        image_assets.add(get_empty_square_image(SquareImageSize::Small)),
        image_assets.add(get_square_image(SquareImageSize::Small, PieceShape::J, 8)),
        image_assets.add(get_square_image(SquareImageSize::Small, PieceShape::T, 2)),
        image_assets.add(get_square_image(SquareImageSize::Small, PieceShape::Z, 8)),
        image_assets.add(get_square_image(SquareImageSize::Small, PieceShape::Z, 9)),
        image_assets.add(get_square_image(SquareImageSize::Small, PieceShape::Z, 0)),
        image_assets.add(get_square_image(SquareImageSize::Small, PieceShape::Z, 1)),
    ];

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
                                    texture: square_images[(*square) as usize].clone(),
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
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<GamepadButton>>,
    controller: Res<Controller>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    if keys.just_pressed(KeyCode::Enter)
        || controller.gamepads.iter().any(|gamepad| {
            buttons.just_pressed(GamepadButton {
                gamepad: *gamepad,
                button_type: GamepadButtonType::Start,
            })
        })
    {
        app_state.set(AppState::Menu);
    }
}
