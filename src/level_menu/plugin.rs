use bevy::{
    color::palettes::css::{BLACK, BLUE, CRIMSON, GOLD, GREEN, WHITE},
    prelude::*,
};

use crate::{
    app_state::AppState,
    audio::plugin::PlaySoundEvent,
    controller::Controller,
    game::player::{PlayerConfig, PlayerData, PlayerState},
    inputs::PlayerInputs,
    utility::despawn_all,
};

pub fn setup(app: &mut App) {
    app.insert_resource(LevelMenuData::default())
        .add_systems(OnEnter(AppState::LevelMenu), setup_screen)
        .add_systems(
            Update,
            (update_ui_system, handle_input_system)
                .chain()
                .run_if(in_state(AppState::LevelMenu)),
        )
        .add_systems(
            OnExit(AppState::LevelMenu),
            despawn_all::<LevelMenuEntityMarker>,
        );
}

#[derive(Component)]
struct LevelMenuEntityMarker;

#[derive(Component)]
struct LevelButtonEntityMarker {
    cordinate: (i32, i32),
}

#[derive(Resource)]
pub struct LevelMenuData {
    selected_level: (i32, i32),
    pub config: PlayerConfig,
}

impl LevelMenuData {
    pub fn new() -> Self {
        Self {
            selected_level: (0, 0),
            config: PlayerConfig::default(),
        }
    }
}

impl Default for LevelMenuData {
    fn default() -> Self {
        Self::new()
    }
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
            LevelMenuEntityMarker,
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
                                row_gap: Val::Px(5.0),
                                column_gap: Val::Px(5.0),
                                border: UiRect::all(Val::Px(5.0)),
                                ..default()
                            },
                            background_color: GREEN.into(),
                            border_color: GREEN.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            for (y, rows) in LEVELS.iter().enumerate() {
                                for (x, col) in rows.iter().enumerate() {
                                    if let Some(level) = col {
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
                                                    background_color: BLUE.into(),
                                                    ..default()
                                                },
                                                LevelButtonEntityMarker {
                                                    cordinate: (x as i32, y as i32),
                                                },
                                            ))
                                            .with_children(|parent| {
                                                parent.spawn(TextBundle::from_section(
                                                    format!("{}", level),
                                                    TextStyle {
                                                        font_size: 40.0,
                                                        color: CRIMSON.into(),
                                                        ..default()
                                                    },
                                                ));
                                            });
                                    } else {
                                        parent.spawn(NodeBundle {
                                            style: Style {
                                                width: Val::Px(60.0),
                                                height: Val::Px(60.0),
                                                align_items: AlignItems::Center,
                                                justify_content: JustifyContent::Center,
                                                ..default()
                                            },
                                            background_color: BLACK.into(),
                                            ..default()
                                        });
                                    }
                                }
                            }
                        });
                });
        });
}

const LEVELS: &'static [[Option<usize>; 5]; 6] = &[
    [Some(0), Some(1), Some(2), Some(3), Some(4)],
    [Some(5), Some(6), Some(7), Some(8), Some(9)],
    [Some(10), Some(11), Some(12), Some(13), Some(14)],
    [Some(15), Some(16), Some(17), Some(18), Some(19)],
    [None, None, None, None, Some(29)],
    [None, None, None, None, Some(39)],
];

const LEVELS_ROWS: usize = LEVELS.len();
const LEVELS_COLS: usize = LEVELS[0].len();

fn update_ui_system(
    time: Res<Time>,
    mut query: Query<(&mut BackgroundColor, &LevelButtonEntityMarker)>,
    level_menu_data: Res<LevelMenuData>,
) {
    fn sin(elapsed: f32) -> f32 {
        const SPEED: f32 = 30.0;
        (((elapsed * SPEED).sin() + 1.0) / 4.0 + 0.5).clamp(0.5, 1.0)
    }

    query.iter_mut().for_each(|(mut bg_color, level_button)| {
        if level_button.cordinate == level_menu_data.selected_level {
            let color = GOLD * sin(time.elapsed_seconds());
            *bg_color = color.into();
        } else {
            *bg_color = BLACK.into();
        }
    });
}

fn handle_input_system(
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<GamepadButton>>,
    controller: Res<Controller>,
    mut level_menu_data: ResMut<LevelMenuData>,
    mut e_play_sound: EventWriter<PlaySoundEvent>,
    mut app_state: ResMut<NextState<AppState>>,
    mut player_state: ResMut<NextState<PlayerState>>,
    mut player_data: ResMut<PlayerData>,
) {
    let inputs =
        PlayerInputs::with_keyboard(&keys) | PlayerInputs::with_gamepads(&buttons, &controller);

    match (inputs.up.0, inputs.down.0) {
        (true, false) => {
            level_menu_data.selected_level.1 =
                (level_menu_data.selected_level.1 - 1).rem_euclid(LEVELS_ROWS as i32);
            if level_menu_data.selected_level.1 >= 4 {
                level_menu_data.selected_level.0 = LEVELS_COLS as i32 - 1;
            }
            e_play_sound.send(PlaySoundEvent::MoveCursor);
        }
        (false, true) => {
            level_menu_data.selected_level.1 =
                (level_menu_data.selected_level.1 + 1).rem_euclid(LEVELS_ROWS as i32);
            if level_menu_data.selected_level.1 >= 4 {
                level_menu_data.selected_level.0 = LEVELS_COLS as i32 - 1;
            }
            e_play_sound.send(PlaySoundEvent::MoveCursor);
        }
        _ => {
            if level_menu_data.selected_level.1 < 4 {
                match (inputs.left.0, inputs.right.0) {
                    (true, false) => {
                        level_menu_data.selected_level.0 =
                            (level_menu_data.selected_level.0 - 1).rem_euclid(LEVELS_COLS as i32);
                        e_play_sound.send(PlaySoundEvent::MoveCursor);
                    }
                    (false, true) => {
                        level_menu_data.selected_level.0 =
                            (level_menu_data.selected_level.0 + 1).rem_euclid(LEVELS_COLS as i32);
                        e_play_sound.send(PlaySoundEvent::MoveCursor);
                    }
                    _ => {}
                }
            }
        }
    }

    if inputs.start {
        if let Some(level) = LEVELS[level_menu_data.selected_level.1 as usize]
            [level_menu_data.selected_level.0 as usize]
        {
            level_menu_data.config.start_level = level;

            *player_data = PlayerData::new(level_menu_data.config);
            e_play_sound.send(PlaySoundEvent::StartGame);
            player_state.set(PlayerState::GameRunning);
            app_state.set(AppState::Game);
        }
    } else if inputs.b.0 {
        e_play_sound.send(PlaySoundEvent::StartGame);
        app_state.set(AppState::GameModeMenu);
    }
}
