use bevy::{
    color::palettes::css::{BLACK, CRIMSON, WHITE},
    prelude::*,
};

use crate::{
    app_state::AppState,
    game::plugin::{PlayerData, PlayerState},
    utility::despawn_all,
};

pub fn setup(app: &mut App) {
    app.add_systems(OnEnter(AppState::Menu), setup_screen)
        .add_systems(Update, menu_action.run_if(in_state(AppState::Menu)))
        .add_systems(OnExit(AppState::Menu), despawn_all::<MenuEntityMarker>);
}

#[derive(Component)]
struct MenuEntityMarker;

fn setup_screen(mut commands: Commands) {
    let button_style = Style {
        width: Val::Px(60.0),
        height: Val::Px(60.0),
        margin: UiRect::all(Val::Px(5.0)),
        padding: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font_size: 40.0,
        color: WHITE.into(),
        ..default()
    };

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
                ..default()
            },
            MenuEntityMarker,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(20.0)),
                        padding: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                    background_color: CRIMSON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            "LEVEL",
                            TextStyle {
                                font_size: 40.0,
                                color: WHITE.into(),
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(20.0)),
                            ..default()
                        }),
                    );

                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Grid,
                                grid_template_columns: vec![GridTrack::auto(); 5],
                                ..default()
                            },
                            background_color: CRIMSON.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            let mut levels: Vec<Option<usize>> = (0..20)
                                .map(|level| Some(level))
                                .collect::<Vec<Option<usize>>>();
                            levels.append(&mut vec![None, None, None, None, Some(29)]);
                            levels.append(&mut vec![None, None, None, None, Some(39)]);

                            for level in levels {
                                if let Some(level) = level {
                                    parent
                                        .spawn((
                                            ButtonBundle {
                                                style: button_style.clone(),
                                                background_color: BLACK.into(),
                                                ..default()
                                            },
                                            MenuButtonAction::Level(level),
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn(TextBundle::from_section(
                                                format!("{}", level),
                                                button_text_style.clone(),
                                            ));
                                        });
                                } else {
                                    // placeholder
                                    parent.spawn((ButtonBundle {
                                        style: button_style.clone(),
                                        background_color: Srgba::new(0.0, 0.0, 0.0, 0.0).into(),
                                        ..default()
                                    },));
                                }
                            }
                        });
                });
        });
}

#[derive(Clone, Copy, Component)]
enum MenuButtonAction {
    Level(usize),
}

fn menu_action(
    q_interaction: Query<(&Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>)>,
    mut app_state: ResMut<NextState<AppState>>,
    mut player_state: ResMut<NextState<PlayerState>>,
    mut player_data: ResMut<PlayerData>,
) {
    for (interaction, menu_button_action) in &q_interaction {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Level(level) => {
                    *player_data = PlayerData::new(*level);
                    player_state.set(PlayerState::GameRunning);
                    app_state.set(AppState::Game);
                }
            }
        }
    }
}
