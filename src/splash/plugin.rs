use bevy::{
    color::palettes::css::{BLACK, CRIMSON, WHITE},
    prelude::*,
};

use crate::{app_state::AppState, utility::despawn_all};

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

fn setup_screen(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: CRIMSON.into(),
                ..default()
            },
            SplashEntityMarker,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(600.0),
                        height: Val::Px(400.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: BLACK.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            "TETRIS",
                            TextStyle {
                                font_size: 120.0,
                                color: WHITE.into(),
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(60.0)),
                            ..default()
                        }),
                    );
                    parent.spawn(
                        TextBundle::from_section(
                            "PRESS START",
                            TextStyle {
                                font_size: 40.0,
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
        });
}

fn handle_input_system(
    q_keys: Res<ButtonInput<KeyCode>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    if q_keys.pressed(KeyCode::Enter) {
        app_state.set(AppState::Menu);
    }
}
