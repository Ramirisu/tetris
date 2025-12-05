use bevy::{
    color::palettes::css::{BLACK, BLUE, GOLD, GREEN, RED, WHITE},
    prelude::*,
};

use crate::{
    app_state::AppState,
    audio::plugin::PlaySoundMessage,
    game_screen::{
        game::{GameConfig, GameState},
        level::Level,
        player::{PlayerData, PlayerPhase},
    },
    input::{controller_mapping::ControllerMapping, player_inputs::PlayerInputs},
    logo::logo,
    settings_menu::scale_factor::{WINDOW_HEIGHT, WINDOW_WIDTH},
    utility::{effect::flicker, entity::despawn_all},
};

pub fn setup(app: &mut App) {
    app.insert_resource(LevelMenuData::default())
        .add_systems(OnEnter(AppState::LevelMenu), setup_screen)
        .add_systems(
            Update,
            (handle_input_system, update_ui_system)
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
}

impl LevelMenuData {
    pub fn new() -> Self {
        Self {
            selected_level: (0, 0),
        }
    }
}

impl Default for LevelMenuData {
    fn default() -> Self {
        Self::new()
    }
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

fn setup_screen(mut commands: Commands, mut image_assets: ResMut<Assets<Image>>) {
    commands
        .spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            LevelMenuEntityMarker,
        ))
        .with_children(|p| {
            p.spawn(Node {
                width: Val::Px(WINDOW_WIDTH),
                height: Val::Px(WINDOW_HEIGHT),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(50.0)),
                ..default()
            })
            .with_children(|p| {
                p.spawn(Node {
                    margin: UiRect::all(Val::Px(40.0)),
                    ..default()
                })
                .with_child(logo(Val::Px(20.0), &mut image_assets));

                p.spawn((
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(10.0)),
                        padding: UiRect::all(Val::Px(10.0)),
                        border: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    BorderColor::from(BLUE),
                ))
                .with_children(|p| {
                    p.spawn((
                        Node {
                            margin: UiRect::all(Val::Px(20.0)),
                            ..default()
                        },
                        Text::new(t!("tetris.level_option.level")),
                        TextFont::from_font_size(40.0),
                        TextColor::from(WHITE),
                    ));

                    p.spawn((
                        Node {
                            display: Display::Grid,
                            grid_template_columns: vec![GridTrack::auto(); 5],
                            justify_items: JustifyItems::Center,
                            align_items: AlignItems::Center,
                            row_gap: Val::Px(5.0),
                            column_gap: Val::Px(5.0),
                            border: UiRect::all(Val::Px(5.0)),
                            ..default()
                        },
                        BackgroundColor::from(GREEN),
                        BorderColor::from(GREEN),
                    ))
                    .with_children(|p| {
                        for (y, rows) in LEVELS.iter().enumerate() {
                            for (x, col) in rows.iter().enumerate() {
                                let mut ec = p.spawn((
                                    Node {
                                        width: Val::Px(60.0),
                                        height: Val::Px(60.0),
                                        display: Display::Flex,
                                        flex_direction: FlexDirection::Column,
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    BackgroundColor::from(BLACK),
                                    LevelButtonEntityMarker {
                                        cordinate: (x as i32, y as i32),
                                    },
                                ));
                                if let Some(level) = col {
                                    ec.with_child((
                                        Text::new(level.to_string()),
                                        TextFont::from_font_size(40.0),
                                        TextColor::from(RED),
                                        TextLayout::new_with_justify(Justify::Center),
                                    ));
                                }
                            }
                        }
                    });
                });
            });
        });
}

fn handle_input_system(
    keys: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    controller_mapping: Res<ControllerMapping>,
    mut level_menu_data: ResMut<LevelMenuData>,
    mut play_sound: MessageWriter<PlaySoundMessage>,
    mut game_config: ResMut<GameConfig>,
    mut app_state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut player_phase: ResMut<NextState<PlayerPhase>>,
    mut player_data: ResMut<PlayerData>,
) {
    let player_inputs = PlayerInputs::with_keyboard(&keys)
        | PlayerInputs::with_gamepads(gamepads, *controller_mapping);

    if player_inputs.soft_reset {
        play_sound.write(PlaySoundMessage::StartGame);
        app_state.set(AppState::SplashScreen);
        return;
    }

    match (
        player_inputs.up.just_pressed,
        player_inputs.down.just_pressed,
    ) {
        (true, false) => {
            level_menu_data.selected_level.1 =
                (level_menu_data.selected_level.1 - 1).rem_euclid(LEVELS_ROWS as i32);
            if level_menu_data.selected_level.1 >= 4 {
                level_menu_data.selected_level.0 = LEVELS_COLS as i32 - 1;
            }
            play_sound.write(PlaySoundMessage::MoveCursor);
        }
        (false, true) => {
            level_menu_data.selected_level.1 =
                (level_menu_data.selected_level.1 + 1).rem_euclid(LEVELS_ROWS as i32);
            if level_menu_data.selected_level.1 >= 4 {
                level_menu_data.selected_level.0 = LEVELS_COLS as i32 - 1;
            }
            play_sound.write(PlaySoundMessage::MoveCursor);
        }
        _ => {
            if level_menu_data.selected_level.1 < 4 {
                match (
                    player_inputs.left.just_pressed,
                    player_inputs.right.just_pressed,
                ) {
                    (true, false) => {
                        level_menu_data.selected_level.0 =
                            (level_menu_data.selected_level.0 - 1).rem_euclid(LEVELS_COLS as i32);
                        play_sound.write(PlaySoundMessage::MoveCursor);
                    }
                    (false, true) => {
                        level_menu_data.selected_level.0 =
                            (level_menu_data.selected_level.0 + 1).rem_euclid(LEVELS_COLS as i32);
                        play_sound.write(PlaySoundMessage::MoveCursor);
                    }
                    _ => {}
                }
            }
        }
    }

    if player_inputs.start.just_pressed {
        if let Some(level) = LEVELS[level_menu_data.selected_level.1 as usize]
            [level_menu_data.selected_level.0 as usize]
        {
            game_config.start_level = Level(level);

            *player_data = PlayerData::new(*game_config);
            play_sound.write(PlaySoundMessage::StartGame);
            game_state.set(GameState::Running);
            player_phase.set(PlayerPhase::Init);
            app_state.set(AppState::Game);
        }
    } else if player_inputs.b.just_pressed {
        play_sound.write(PlaySoundMessage::StartGame);
        app_state.set(AppState::SettingsMenu);
    }
}

fn update_ui_system(
    t: Res<Time>,
    q: Query<(&mut BackgroundColor, &LevelButtonEntityMarker)>,
    level_menu_data: Res<LevelMenuData>,
) {
    for (mut bg_color, marker) in q {
        if marker.cordinate == level_menu_data.selected_level {
            let mut color = GOLD;
            color.set_alpha(flicker(t.elapsed_secs(), 0.25));

            *bg_color = color.into();
        } else {
            *bg_color = BLACK.into();
        }
    }
}
