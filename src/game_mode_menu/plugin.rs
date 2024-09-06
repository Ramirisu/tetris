use bevy::{
    color::palettes::css::{BLUE, GREEN, RED, WHITE},
    prelude::*,
};

use crate::{
    app_state::AppState, audio::plugin::PlaySoundEvent, game::transition::Transition,
    inputs::PlayerInputs, level_menu::plugin::LevelMenuData, utility::despawn_all,
};

pub fn setup(app: &mut App) {
    app.insert_resource(GameModeMenuData::default())
        .add_systems(OnEnter(AppState::GameModeMenu), setup_screen)
        .add_systems(
            Update,
            (update_ui_system, handle_input_system)
                .chain()
                .run_if(in_state(AppState::GameModeMenu)),
        )
        .add_systems(
            OnExit(AppState::GameModeMenu),
            despawn_all::<GameModeMenuEntityMarker>,
        );
}

#[derive(Component)]
struct GameModeMenuEntityMarker;

#[derive(Component)]
struct GameModeEntityMarker(i32);

#[derive(Component)]
struct GameFeatureEntityMarker;

#[derive(Resource, Default)]
struct GameModeMenuData {
    selected_index: i32,
}

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
                ..default()
            },
            GameModeMenuEntityMarker,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(10.0)),
                        padding: UiRect::all(Val::Px(10.0)),
                        border: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    border_color: BLUE.into(),
                    border_radius: BorderRadius::all(Val::Px(5.0)),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            "Select Game Mode",
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
                                display: Display::Flex,
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                column_gap: Val::Px(20.0),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        display: Display::Grid,
                                        grid_template_columns: vec![GridTrack::auto(); 2],
                                        border: UiRect::all(Val::Px(5.0)),
                                        ..default()
                                    },
                                    border_color: GREEN.into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    for (index, game_mode) in GAME_MODES.iter().enumerate() {
                                        parent
                                            .spawn((
                                                NodeBundle {
                                                    style: Style {
                                                        width: Val::Px(60.0),
                                                        height: Val::Px(60.0),
                                                        align_items: AlignItems::Center,
                                                        justify_content: JustifyContent::Center,
                                                        ..default()
                                                    },
                                                    visibility: Visibility::Hidden,
                                                    ..default()
                                                },
                                                GameModeEntityMarker(index as i32),
                                            ))
                                            .with_children(|parent| {
                                                parent.spawn(TextBundle::from_section(
                                                    "*",
                                                    TextStyle {
                                                        font_size: 40.0,
                                                        color: WHITE.into(),
                                                        ..default()
                                                    },
                                                ));
                                            });
                                        parent
                                            .spawn(NodeBundle {
                                                style: Style {
                                                    width: Val::Px(360.0),
                                                    height: Val::Px(60.0),
                                                    align_items: AlignItems::Center,
                                                    justify_content: JustifyContent::Start,
                                                    ..default()
                                                },
                                                ..default()
                                            })
                                            .with_children(|parent| {
                                                parent.spawn(TextBundle::from_section(
                                                    game_mode.name,
                                                    TextStyle {
                                                        font_size: 40.0,
                                                        color: WHITE.into(),
                                                        ..default()
                                                    },
                                                ));
                                            });
                                    }
                                });

                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        display: Display::Grid,
                                        grid_template_columns: vec![GridTrack::auto(); 2],
                                        border: UiRect::all(Val::Px(5.0)),
                                        padding: UiRect::all(Val::Px(20.0)),
                                        ..default()
                                    },
                                    border_color: WHITE.into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn((
                                        TextBundle::from_sections([
                                            TextSection {
                                                value: "  TRANSITION: ".into(),
                                                style: TextStyle {
                                                    font_size: 30.0,
                                                    color: WHITE.into(),
                                                    ..default()
                                                },
                                                ..default()
                                            },
                                            TextSection::from_style(TextStyle {
                                                font_size: 30.0,
                                                color: WHITE.into(),
                                                ..default()
                                            }),
                                            TextSection {
                                                value: "LV39 LINECAP: ".into(),
                                                style: TextStyle {
                                                    font_size: 30.0,
                                                    color: WHITE.into(),
                                                    ..default()
                                                },
                                                ..default()
                                            },
                                            TextSection::from_style(TextStyle {
                                                font_size: 30.0,
                                                color: WHITE.into(),
                                                ..default()
                                            }),
                                        ]),
                                        GameFeatureEntityMarker,
                                    ));
                                });
                        });
                });
        });
}

struct GameMode {
    name: &'static str,
    transition: Transition,
    lv39_linecap: bool,
}

const GAME_MODES: [GameMode; 3] = [
    GameMode {
        name: "CLASSIC",
        transition: Transition::Classic,
        lv39_linecap: false,
    },
    GameMode {
        name: "COMPETITIVE",
        transition: Transition::Classic,
        lv39_linecap: true,
    },
    GameMode {
        name: "TRANSITION",
        transition: Transition::Fast,
        lv39_linecap: false,
    },
];

fn update_ui_system(
    mut query: ParamSet<(
        Query<(&mut Visibility, &GameModeEntityMarker)>,
        Query<&mut Text, With<GameFeatureEntityMarker>>,
    )>,
    game_mode_menu_data: Res<GameModeMenuData>,
) {
    query.p0().iter_mut().for_each(|(mut visibility, index)| {
        if game_mode_menu_data.selected_index == index.0 {
            *visibility = Visibility::Inherited;
        } else {
            *visibility = Visibility::Hidden;
        }
    });

    fn format(msg: impl std::fmt::Display) -> String {
        format!("{:<8}\n", msg)
    }

    if let Ok(mut text) = query.p1().get_single_mut() {
        match GAME_MODES[game_mode_menu_data.selected_index as usize].transition {
            Transition::Classic => {
                text.sections[1].value = format(Transition::Classic.to_string().to_uppercase());
                text.sections[1].style.color = GREEN.into();
            }
            Transition::Fast => {
                text.sections[1].value = format(Transition::Fast.to_string().to_uppercase());
                text.sections[1].style.color = RED.into();
            }
        }
        if GAME_MODES[game_mode_menu_data.selected_index as usize].lv39_linecap {
            text.sections[3].value = format("ON");
            text.sections[3].style.color = GREEN.into();
        } else {
            text.sections[3].value = format("OFF");
            text.sections[3].style.color = RED.into();
        }
    }
}

fn handle_input_system(
    player_inputs: Res<PlayerInputs>,
    mut game_mode_menu_data: ResMut<GameModeMenuData>,
    mut e_play_sound: EventWriter<PlaySoundEvent>,
    mut app_state: ResMut<NextState<AppState>>,
    mut level_menu_data: ResMut<LevelMenuData>,
) {
    match (player_inputs.up.0, player_inputs.down.0) {
        (true, false) => {
            game_mode_menu_data.selected_index =
                (game_mode_menu_data.selected_index - 1).rem_euclid(GAME_MODES.len() as i32);
            e_play_sound.send(PlaySoundEvent::MoveCursor);
        }
        (false, true) => {
            game_mode_menu_data.selected_index =
                (game_mode_menu_data.selected_index + 1).rem_euclid(GAME_MODES.len() as i32);
            e_play_sound.send(PlaySoundEvent::MoveCursor);
        }
        _ => {
            if player_inputs.start {
                level_menu_data.config.lv39_linecap =
                    GAME_MODES[game_mode_menu_data.selected_index as usize].lv39_linecap;
                level_menu_data.config.transition =
                    GAME_MODES[game_mode_menu_data.selected_index as usize].transition;
                e_play_sound.send(PlaySoundEvent::StartGame);
                app_state.set(AppState::LevelMenu);
            } else if player_inputs.b.0 {
                e_play_sound.send(PlaySoundEvent::StartGame);
                app_state.set(AppState::Splash);
            }
        }
    }
}
