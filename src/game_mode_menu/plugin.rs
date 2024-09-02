use bevy::{
    color::palettes::css::{BLUE, GREEN, RED, WHITE},
    prelude::*,
};

use crate::{
    app_state::AppState, audio::plugin::PlaySoundEvent, controller::Controller,
    level_menu::plugin::LevelMenuData, utility::despawn_all,
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
                                        ..default()
                                    },
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
                                        ..default()
                                    },
                                    border_color: WHITE.into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent
                                        .spawn((NodeBundle {
                                            style: Style {
                                                width: Val::Px(360.0),
                                                height: Val::Px(60.0),
                                                align_items: AlignItems::Center,
                                                justify_content: JustifyContent::Center,
                                                ..default()
                                            },
                                            ..default()
                                        },))
                                        .with_children(|parent| {
                                            parent.spawn((
                                                TextBundle::from_sections([
                                                    TextSection {
                                                        value: "LV39 LINECAP ".into(),
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
        });
}

struct GameMode {
    name: &'static str,
    lv39_linecap: bool,
}

const GAME_MODES: [GameMode; 2] = [
    GameMode {
        name: "Classic",
        lv39_linecap: false,
    },
    GameMode {
        name: "Competitive",
        lv39_linecap: true,
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
    if let Ok(mut text) = query.p1().get_single_mut() {
        if GAME_MODES[game_mode_menu_data.selected_index as usize].lv39_linecap {
            text.sections[1].value = format!("{:<8}", "ON");
            text.sections[1].style.color = GREEN.into();
        } else {
            text.sections[1].value = format!("{:<8}", "OFF");
            text.sections[1].style.color = RED.into();
        }
    }
}

pub struct GameModeMenuInputs {
    up: bool,
    down: bool,
    start: bool,
}

impl std::ops::BitOrAssign for GameModeMenuInputs {
    fn bitor_assign(&mut self, rhs: Self) {
        self.up |= rhs.up;
        self.down |= rhs.down;
        self.start |= rhs.start;
    }
}

fn handle_input_system(
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<GamepadButton>>,
    controller: Res<Controller>,
    mut game_mode_menu_data: ResMut<GameModeMenuData>,
    mut e_play_sound: EventWriter<PlaySoundEvent>,
    mut app_state: ResMut<NextState<AppState>>,
    mut level_menu_data: ResMut<LevelMenuData>,
) {
    let mut inputs = GameModeMenuInputs {
        up: keys.just_pressed(KeyCode::ArrowUp),
        down: keys.just_pressed(KeyCode::ArrowDown),
        start: keys.just_pressed(KeyCode::Enter),
    };

    for gamepad in &controller.gamepads {
        inputs |= GameModeMenuInputs {
            up: buttons.just_pressed(GamepadButton {
                gamepad: *gamepad,
                button_type: GamepadButtonType::DPadUp,
            }),
            down: buttons.just_pressed(GamepadButton {
                gamepad: *gamepad,
                button_type: GamepadButtonType::DPadDown,
            }),
            start: buttons.just_pressed(GamepadButton {
                gamepad: *gamepad,
                button_type: GamepadButtonType::Start,
            }),
        };
    }

    match (inputs.up, inputs.down) {
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
            if inputs.start {
                level_menu_data.config.lv39_linecap =
                    GAME_MODES[game_mode_menu_data.selected_index as usize].lv39_linecap;
                e_play_sound.send(PlaySoundEvent::StartGame);
                app_state.set(AppState::LevelMenu);
            }
        }
    }
}
