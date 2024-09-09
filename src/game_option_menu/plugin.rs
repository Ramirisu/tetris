use bevy::{color::palettes::css::WHITE, prelude::*};

use crate::{
    app_state::AppState,
    audio::plugin::PlaySoundEvent,
    game::{drop_speed::DropSpeed, transition::Transition},
    inputs::PlayerInputs,
    level_menu::plugin::LevelMenuData,
    logo::{load_logo_images, TETRIS_BITMAP},
    utility::despawn_all,
};

#[cfg(not(target_arch = "wasm32"))]
use bevy::window::WindowMode;

pub fn setup(app: &mut App) {
    app.insert_resource(GameOptionMenuData::default())
        .add_systems(OnEnter(AppState::GameModeMenu), setup_screen)
        .add_systems(
            Update,
            (update_ui_system, handle_input_system)
                .chain()
                .run_if(in_state(AppState::GameModeMenu)),
        )
        .add_systems(
            OnExit(AppState::GameModeMenu),
            despawn_all::<GameOptionMenuEntityMarker>,
        );
}

#[derive(Component)]
struct GameOptionEntityMarker(pub GameOptionMenuState);

#[derive(Component)]
struct GameOptionMenuEntityMarker;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum GameOptionMenuState {
    #[default]
    Tetris,
    Transition,
    Linecap,
    DropSpeed,
    #[cfg(not(target_arch = "wasm32"))]
    WindowMode,
}

impl GameOptionMenuState {
    pub fn iter() -> std::slice::Iter<'static, GameOptionMenuState> {
        #[cfg(not(target_arch = "wasm32"))]
        type ArrayType = [GameOptionMenuState; 5];
        #[cfg(target_arch = "wasm32")]
        type ArrayType = [GameOptionMenuState; 4];
        const STATES: ArrayType = [
            GameOptionMenuState::Tetris,
            GameOptionMenuState::Transition,
            GameOptionMenuState::Linecap,
            GameOptionMenuState::DropSpeed,
            #[cfg(not(target_arch = "wasm32"))]
            GameOptionMenuState::WindowMode,
        ];
        STATES.iter()
    }
}

#[derive(Resource)]
struct GameOptionMenuData {
    state: GameOptionMenuState,
    transition: Transition,
    lv39_linecap: bool,
    drop_speed: DropSpeed,
    #[cfg(not(target_arch = "wasm32"))]
    window_mode: WindowMode,
}

impl GameOptionMenuData {
    pub fn new() -> Self {
        Self {
            state: GameOptionMenuState::default(),
            transition: Transition::Default,
            lv39_linecap: false,
            drop_speed: DropSpeed::Level,
            #[cfg(not(target_arch = "wasm32"))]
            window_mode: WindowMode::Windowed,
        }
    }
}

impl Default for GameOptionMenuData {
    fn default() -> Self {
        Self::new()
    }
}

fn setup_screen(mut commands: Commands, mut image_assets: ResMut<Assets<Image>>) {
    let logo_images = load_logo_images(&mut image_assets);

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            GameOptionMenuEntityMarker,
        ))
        .with_children(|parent| {
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
                                        width: Val::Px(24.0),
                                        height: Val::Px(24.0),
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

            parent
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::all(Val::Px(40.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    for state in GameOptionMenuState::iter() {
                        parent.spawn((
                            TextBundle::from_sections(vec![
                                TextSection::from_style(TextStyle {
                                    font_size: 40.0,
                                    color: WHITE.into(),
                                    ..default()
                                }),
                                TextSection::from_style(TextStyle {
                                    font_size: 40.0,
                                    color: WHITE.into(),
                                    ..default()
                                }),
                            ]),
                            GameOptionEntityMarker(*state),
                        ));
                    }
                });
        });
}

fn update_ui_system(
    mut query: Query<(&mut Text, &GameOptionEntityMarker)>,
    game_option_menu_data: Res<GameOptionMenuData>,
) {
    query.iter_mut().for_each(|(mut text, marker)| {
        let selected = marker.0 == game_option_menu_data.state;
        let fname = |name| -> String {
            let s = if selected { ">" } else { " " };
            format!("{} {:16}", s, name)
        };
        let fopt = |name, l, r| -> String {
            let l = if l { "<" } else { " " };
            let r = if r { ">" } else { " " };
            format!("{} {:12} {}", l, name, r)
        };
        match marker.0 {
            GameOptionMenuState::Tetris => {
                text.sections[0].value = fname("TETRIS");
                text.sections[1].value = fopt("", false, false);
            }
            GameOptionMenuState::Transition => {
                text.sections[0].value = fname("TRANSITION");
                match game_option_menu_data.transition {
                    Transition::Default => text.sections[1].value = fopt("DEFAULT", false, true),
                    Transition::Every10Lines => {
                        text.sections[1].value = fopt("10 LINES", true, false)
                    }
                    Transition::Every4Lines => {
                        text.sections[1].value = fopt(" 4 LINES", true, false)
                    }
                };
            }
            GameOptionMenuState::Linecap => {
                text.sections[0].value = fname("LV39 LINECAP");
                if game_option_menu_data.lv39_linecap {
                    text.sections[1].value = fopt("ON", true, false);
                } else {
                    text.sections[1].value = fopt("OFF", false, true);
                }
            }
            GameOptionMenuState::DropSpeed => {
                text.sections[0].value = fname("DROPSPEED");
                match game_option_menu_data.drop_speed {
                    DropSpeed::Level => text.sections[1].value = fopt("LEVEL", false, true),
                    DropSpeed::Locked => text.sections[1].value = fopt("LOCKED", true, false),
                };
            }
            #[cfg(not(target_arch = "wasm32"))]
            GameOptionMenuState::WindowMode => {
                text.sections[0].value = fname("WINDOW MODE");
                match game_option_menu_data.window_mode {
                    WindowMode::Windowed => {
                        text.sections[1].value = fopt("WINDOWED", false, true);
                    }
                    WindowMode::BorderlessFullscreen => {
                        text.sections[1].value = fopt("BORDERLESS", true, true);
                    }
                    WindowMode::Fullscreen => {
                        text.sections[1].value = fopt("FULLSCREEN", true, false);
                    }
                    _ => (),
                };
            }
        }
    });
}

fn handle_input_system(
    player_inputs: Res<PlayerInputs>,
    mut game_option_menu_data: ResMut<GameOptionMenuData>,
    mut level_menu_data: ResMut<LevelMenuData>,
    mut app_state: ResMut<NextState<AppState>>,
    mut e_play_sound: EventWriter<PlaySoundEvent>,
    #[cfg(not(target_arch = "wasm32"))] mut window: Query<&mut Window>,
) {
    if player_inputs.b.0 {
        app_state.set(AppState::Splash);
        e_play_sound.send(PlaySoundEvent::StartGame);
    }

    match game_option_menu_data.state {
        GameOptionMenuState::Tetris => {
            if player_inputs.up.0 {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    game_option_menu_data.state = GameOptionMenuState::WindowMode;
                }
                #[cfg(target_arch = "wasm32")]
                {
                    game_option_menu_data.state = GameOptionMenuState::Linecap;
                }
                e_play_sound.send(PlaySoundEvent::MoveCursor);
            } else if player_inputs.down.0 {
                game_option_menu_data.state = GameOptionMenuState::Transition;
                e_play_sound.send(PlaySoundEvent::MoveCursor);
            } else if player_inputs.start {
                level_menu_data.config.transition = game_option_menu_data.transition;
                level_menu_data.config.lv39_linecap = game_option_menu_data.lv39_linecap;
                level_menu_data.config.drop_speed = game_option_menu_data.drop_speed;
                e_play_sound.send(PlaySoundEvent::StartGame);
                app_state.set(AppState::LevelMenu);
            }
        }
        GameOptionMenuState::Transition => {
            if player_inputs.up.0 {
                game_option_menu_data.state = GameOptionMenuState::Tetris;
                e_play_sound.send(PlaySoundEvent::MoveCursor);
            } else if player_inputs.down.0 {
                game_option_menu_data.state = GameOptionMenuState::Linecap;
                e_play_sound.send(PlaySoundEvent::MoveCursor);
            }
            match game_option_menu_data.transition {
                Transition::Default => {
                    if player_inputs.right.0 {
                        game_option_menu_data.transition = Transition::Every10Lines;
                        e_play_sound.send(PlaySoundEvent::MoveCursor);
                    }
                }
                Transition::Every10Lines => {
                    if player_inputs.right.0 {
                        game_option_menu_data.transition = Transition::Every4Lines;
                        e_play_sound.send(PlaySoundEvent::MoveCursor);
                    } else if player_inputs.left.0 {
                        game_option_menu_data.transition = Transition::Default;
                        e_play_sound.send(PlaySoundEvent::MoveCursor);
                    }
                }
                Transition::Every4Lines => {
                    if player_inputs.left.0 {
                        game_option_menu_data.transition = Transition::Every10Lines;
                        e_play_sound.send(PlaySoundEvent::MoveCursor);
                    }
                }
            }
        }
        GameOptionMenuState::Linecap => {
            if player_inputs.up.0 {
                game_option_menu_data.state = GameOptionMenuState::Transition;
                e_play_sound.send(PlaySoundEvent::MoveCursor);
            } else if player_inputs.down.0 {
                game_option_menu_data.state = GameOptionMenuState::DropSpeed;
                e_play_sound.send(PlaySoundEvent::MoveCursor);
            }
            if player_inputs.right.0 {
                game_option_menu_data.lv39_linecap = true;
                e_play_sound.send(PlaySoundEvent::MoveCursor);
            } else if player_inputs.left.0 {
                game_option_menu_data.lv39_linecap = false;
                e_play_sound.send(PlaySoundEvent::MoveCursor);
            }
        }
        GameOptionMenuState::DropSpeed => {
            if player_inputs.up.0 {
                game_option_menu_data.state = GameOptionMenuState::Linecap;
                e_play_sound.send(PlaySoundEvent::MoveCursor);
            } else if player_inputs.down.0 {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    game_option_menu_data.state = GameOptionMenuState::WindowMode;
                }
                #[cfg(target_arch = "wasm32")]
                {
                    game_option_menu_data.state = GameOptionMenuState::Tetris;
                }
                e_play_sound.send(PlaySoundEvent::MoveCursor);
            }
            if player_inputs.right.0 {
                game_option_menu_data.drop_speed = DropSpeed::Locked;
                e_play_sound.send(PlaySoundEvent::MoveCursor);
            } else if player_inputs.left.0 {
                game_option_menu_data.drop_speed = DropSpeed::Level;
                e_play_sound.send(PlaySoundEvent::MoveCursor);
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        GameOptionMenuState::WindowMode => {
            if player_inputs.up.0 {
                game_option_menu_data.state = GameOptionMenuState::DropSpeed;
                e_play_sound.send(PlaySoundEvent::MoveCursor);
            } else if player_inputs.down.0 {
                game_option_menu_data.state = GameOptionMenuState::Tetris;
                e_play_sound.send(PlaySoundEvent::MoveCursor);
            }
            match game_option_menu_data.window_mode {
                WindowMode::Windowed => {
                    if player_inputs.right.0 {
                        game_option_menu_data.window_mode = WindowMode::BorderlessFullscreen;
                        e_play_sound.send(PlaySoundEvent::MoveCursor);
                    }
                }
                WindowMode::BorderlessFullscreen => {
                    if player_inputs.right.0 {
                        game_option_menu_data.window_mode = WindowMode::Fullscreen;
                        e_play_sound.send(PlaySoundEvent::MoveCursor);
                    } else if player_inputs.left.0 {
                        game_option_menu_data.window_mode = WindowMode::Windowed;
                        e_play_sound.send(PlaySoundEvent::MoveCursor);
                    }
                }
                WindowMode::Fullscreen => {
                    if player_inputs.left.0 {
                        game_option_menu_data.window_mode = WindowMode::BorderlessFullscreen;
                        e_play_sound.send(PlaySoundEvent::MoveCursor);
                    }
                }
                _ => (),
            }

            let mut window = window.single_mut();
            window.mode = game_option_menu_data.window_mode;
        }
    }
}
