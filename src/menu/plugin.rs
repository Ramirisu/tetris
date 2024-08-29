use bevy::{
    color::palettes::css::{BLUE, CRIMSON, GOLD, GREEN, WHITE},
    prelude::*,
};

use crate::{
    app_state::AppState,
    controller::Controller,
    game::plugin::{PlayerData, PlayerState},
    utility::despawn_all,
};

pub fn setup(app: &mut App) {
    app.insert_resource(MenuData::default())
        .add_event::<PlaySoundEvent>()
        .add_systems(
            OnEnter(AppState::Menu),
            (load_audio_assets, setup_screen).chain(),
        )
        .add_systems(
            Update,
            (style_system, handle_input_system, play_sound_system)
                .chain()
                .run_if(in_state(AppState::Menu)),
        )
        .add_systems(
            OnExit(AppState::Menu),
            (despawn_all::<MenuEntityMarker>, unload_audio_assets).chain(),
        );
}

#[derive(Resource)]
struct AudioAssets {
    move_cursor: Handle<AudioSource>,
    start_game: Handle<AudioSource>,
}

#[derive(Component)]
struct MenuEntityMarker;

#[derive(Component)]
struct LevelButtonEntityMarker {
    cordinate: (i32, i32),
}

#[derive(Event)]
enum PlaySoundEvent {
    MoveCurosr,
    StartGame,
}

#[derive(Resource)]
struct MenuData {
    selected_level: (i32, i32),
}

impl MenuData {
    pub fn new() -> Self {
        Self {
            selected_level: (0, 0),
        }
    }
}

impl Default for MenuData {
    fn default() -> Self {
        Self::new()
    }
}

fn setup_screen(mut commands: Commands) {
    let button_style = Style {
        width: Val::Px(60.0),
        height: Val::Px(60.0),
        margin: UiRect::all(Val::Px(5.0)),
        padding: UiRect::all(Val::Px(20.0)),
        border: UiRect::all(Val::Px(5.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font_size: 40.0,
        color: CRIMSON.into(),
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
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            for (y, rows) in LEVELS.iter().enumerate() {
                                for (x, col) in rows.iter().enumerate() {
                                    if let Some(level) = col {
                                        parent
                                            .spawn((
                                                ButtonBundle {
                                                    style: button_style.clone(),
                                                    border_color: GREEN.into(),
                                                    ..default()
                                                },
                                                LevelButtonEntityMarker {
                                                    cordinate: (x as i32, y as i32),
                                                },
                                            ))
                                            .with_children(|parent| {
                                                parent.spawn(TextBundle::from_section(
                                                    format!("{}", level),
                                                    button_text_style.clone(),
                                                ));
                                            });
                                    } else {
                                        parent.spawn((ButtonBundle {
                                            style: button_style.clone(),
                                            ..default()
                                        },));
                                    }
                                }
                            }
                        });
                });
        });
}

fn load_audio_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(AudioAssets {
        move_cursor: asset_server.load("sound/sfx02.wav"),
        start_game: asset_server.load("sound/sfx03.wav"),
    });
}

fn unload_audio_assets(mut commands: Commands) {
    commands.remove_resource::<AudioAssets>();
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

fn play_sound_system(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    mut event_reader: EventReader<PlaySoundEvent>,
) {
    for event in event_reader.read() {
        let audio = match event {
            PlaySoundEvent::MoveCurosr => audio_assets.move_cursor.clone(),
            PlaySoundEvent::StartGame => audio_assets.start_game.clone(),
        };
        commands.spawn(AudioBundle {
            source: audio,
            settings: PlaybackSettings::DESPAWN,
        });
    }
}

fn style_system(
    time: Res<Time>,
    mut query: Query<(
        &mut BorderColor,
        &mut BackgroundColor,
        &LevelButtonEntityMarker,
    )>,
    menu_data: Res<MenuData>,
) {
    fn sin(elapsed: f32) -> f32 {
        const SPEED: f32 = 30.0;
        (((elapsed * SPEED).sin() + 1.0) / 4.0 + 0.5).clamp(0.5, 1.0)
    }

    query
        .iter_mut()
        .for_each(|(mut border_color, mut bg_color, level_button)| {
            if level_button.cordinate == menu_data.selected_level {
                let color = GOLD * sin(time.elapsed_seconds());
                *border_color = GREEN.into();
                *bg_color = color.into();
            } else {
                *border_color = GREEN.into();
                *bg_color = BackgroundColor::DEFAULT;
            }
        });
}

pub struct MenuInputs {
    left: bool,
    right: bool,
    up: bool,
    down: bool,
    start: bool,
}

impl std::ops::BitOrAssign for MenuInputs {
    fn bitor_assign(&mut self, rhs: Self) {
        self.left |= rhs.left;
        self.right |= rhs.right;
        self.up |= rhs.up;
        self.down |= rhs.down;
        self.start |= rhs.start;
    }
}

fn handle_input_system(
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<GamepadButton>>,
    controller: Res<Controller>,
    mut menu_data: ResMut<MenuData>,
    mut e_play_sound: EventWriter<PlaySoundEvent>,
    mut app_state: ResMut<NextState<AppState>>,
    mut player_state: ResMut<NextState<PlayerState>>,
    mut player_data: ResMut<PlayerData>,
) {
    let mut inputs = MenuInputs {
        left: keys.just_pressed(KeyCode::ArrowLeft),
        right: keys.just_pressed(KeyCode::ArrowRight),
        up: keys.just_pressed(KeyCode::ArrowUp),
        down: keys.just_pressed(KeyCode::ArrowDown),
        start: keys.just_pressed(KeyCode::Enter),
    };

    for gamepad in &controller.gamepads {
        inputs |= MenuInputs {
            left: buttons.just_pressed(GamepadButton {
                gamepad: *gamepad,
                button_type: GamepadButtonType::DPadLeft,
            }),
            right: buttons.just_pressed(GamepadButton {
                gamepad: *gamepad,
                button_type: GamepadButtonType::DPadRight,
            }),
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
            menu_data.selected_level.1 =
                (menu_data.selected_level.1 - 1).rem_euclid(LEVELS_ROWS as i32);
            if menu_data.selected_level.1 >= 4 {
                menu_data.selected_level.0 = LEVELS_COLS as i32 - 1;
            }
            e_play_sound.send(PlaySoundEvent::MoveCurosr);
        }
        (false, true) => {
            menu_data.selected_level.1 =
                (menu_data.selected_level.1 + 1).rem_euclid(LEVELS_ROWS as i32);
            if menu_data.selected_level.1 >= 4 {
                menu_data.selected_level.0 = LEVELS_COLS as i32 - 1;
            }
            e_play_sound.send(PlaySoundEvent::MoveCurosr);
        }
        _ => {
            if menu_data.selected_level.1 < 4 {
                match (inputs.left, inputs.right) {
                    (true, false) => {
                        menu_data.selected_level.0 =
                            (menu_data.selected_level.0 - 1).rem_euclid(LEVELS_COLS as i32);
                        e_play_sound.send(PlaySoundEvent::MoveCurosr);
                    }
                    (false, true) => {
                        menu_data.selected_level.0 =
                            (menu_data.selected_level.0 + 1).rem_euclid(LEVELS_COLS as i32);
                        e_play_sound.send(PlaySoundEvent::MoveCurosr);
                    }
                    _ => {}
                }
            }
        }
    }

    if inputs.start {
        if let Some(level) =
            LEVELS[menu_data.selected_level.1 as usize][menu_data.selected_level.0 as usize]
        {
            *player_data = PlayerData::new(level);
            e_play_sound.send(PlaySoundEvent::StartGame);
            player_state.set(PlayerState::GameRunning);
            app_state.set(AppState::Game);
        }
    }
}
