use bevy::{color::palettes::css::WHITE, prelude::*};

use crate::{
    app_state::AppState,
    audio::plugin::PlaySoundEvent,
    controller::Controller,
    game::game::GameConfig,
    input::{controller_mapping::ControllerMapping, player_inputs::PlayerInputs},
    logo::{load_logo_images, TETRIS_BITMAP},
    scale::plugin::ScaleFactor,
    utility::despawn_all,
};

use super::transform::GameOptionMenuTransform;

#[cfg(not(target_arch = "wasm32"))]
use super::{fps_limiter::FPSLimiter, window_mode::WindowMode};

pub fn setup(app: &mut App) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        app.add_plugins(bevy_framepace::FramepacePlugin)
            .add_systems(Startup, init_framepace_settings);
    }
    app.insert_resource(GameOptionMenuTransform::default())
        .insert_resource(GameOptionMenuData::default())
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

#[cfg(not(target_arch = "wasm32"))]
fn init_framepace_settings(mut framepace_settins: ResMut<bevy_framepace::FramepaceSettings>) {
    *framepace_settins = bevy_framepace::FramepaceSettings {
        limiter: FPSLimiter::default().get_limiter(),
    };
}

#[derive(Component)]
struct GameOptionEntityMarker(pub GameOptionMenuSelection);

#[derive(Component)]
struct GameOptionMenuEntityMarker;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum GameOptionMenuSelection {
    #[default]
    Tetris,
    BlankLine0,
    GameOptionsCategory,
    BlankLine1,
    Transition,
    Linecap,
    Gravity,
    Seed,
    Scoring,
    TVSystem,
    NextPieceHint,
    DASCounter,
    ControllerMapping,
    BlankLine2,
    VideoOptionsCategory,
    BlankLine3,
    ScaleFactor,
    #[cfg(not(target_arch = "wasm32"))]
    FPSLimiter,
    #[cfg(not(target_arch = "wasm32"))]
    WindowMode,
}

impl GameOptionMenuSelection {
    pub fn iter() -> std::slice::Iter<'static, GameOptionMenuSelection> {
        #[cfg(not(target_arch = "wasm32"))]
        type ArrayType = [GameOptionMenuSelection; 19];
        #[cfg(target_arch = "wasm32")]
        type ArrayType = [GameOptionMenuSelection; 17];
        const STATES: ArrayType = [
            GameOptionMenuSelection::Tetris,
            GameOptionMenuSelection::BlankLine0,
            GameOptionMenuSelection::GameOptionsCategory,
            GameOptionMenuSelection::BlankLine1,
            GameOptionMenuSelection::Transition,
            GameOptionMenuSelection::Linecap,
            GameOptionMenuSelection::Gravity,
            GameOptionMenuSelection::Seed,
            GameOptionMenuSelection::Scoring,
            GameOptionMenuSelection::TVSystem,
            GameOptionMenuSelection::NextPieceHint,
            GameOptionMenuSelection::DASCounter,
            GameOptionMenuSelection::ControllerMapping,
            GameOptionMenuSelection::BlankLine2,
            GameOptionMenuSelection::VideoOptionsCategory,
            GameOptionMenuSelection::BlankLine3,
            GameOptionMenuSelection::ScaleFactor,
            #[cfg(not(target_arch = "wasm32"))]
            GameOptionMenuSelection::FPSLimiter,
            #[cfg(not(target_arch = "wasm32"))]
            GameOptionMenuSelection::WindowMode,
        ];
        STATES.iter()
    }
}

#[derive(Resource)]
struct GameOptionMenuData {
    selection: GameOptionMenuSelection,
    #[cfg(not(target_arch = "wasm32"))]
    fps_limiter: FPSLimiter,
    #[cfg(not(target_arch = "wasm32"))]
    window_mode: WindowMode,
}

impl GameOptionMenuData {
    pub fn new() -> Self {
        Self {
            selection: GameOptionMenuSelection::default(),
            #[cfg(not(target_arch = "wasm32"))]
            fps_limiter: FPSLimiter::default(),
            #[cfg(not(target_arch = "wasm32"))]
            window_mode: WindowMode::default(),
        }
    }
}

impl Default for GameOptionMenuData {
    fn default() -> Self {
        Self::new()
    }
}

fn setup_screen(
    mut commands: Commands,
    mut image_assets: ResMut<Assets<Image>>,
    game_option_menu_transform: Res<GameOptionMenuTransform>,
) {
    let logo_images = load_logo_images(&mut image_assets);
    let scale = game_option_menu_transform.scale();

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
                        margin: UiRect::all(Val::Px(scale * 40.0)),
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
                                        width: Val::Px(scale * 24.0),
                                        height: Val::Px(scale * 24.0),
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
                        margin: UiRect::all(Val::Px(scale * 40.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    for selection in GameOptionMenuSelection::iter() {
                        parent.spawn((
                            TextBundle::from_sections(vec![
                                TextSection::from_style(TextStyle {
                                    font_size: scale * 36.0,
                                    color: WHITE.into(),
                                    ..default()
                                }),
                                TextSection::from_style(TextStyle {
                                    font_size: scale * 36.0,
                                    color: WHITE.into(),
                                    ..default()
                                }),
                            ]),
                            GameOptionEntityMarker(*selection),
                        ));
                    }
                });
        });
}

fn update_ui_system(
    mut query: Query<(&mut Text, &GameOptionEntityMarker)>,
    game_option_menu_data: Res<GameOptionMenuData>,
    game_config: Res<GameConfig>,
    controller_mapping: Res<ControllerMapping>,
    scale_factor: Res<ScaleFactor>,
    app_state: Res<State<AppState>>,
) {
    if !app_state.is_changed()
        && !game_option_menu_data.is_changed()
        && !game_config.is_changed()
        && !controller_mapping.is_changed()
        && !scale_factor.is_changed()
    {
        return;
    }

    query.iter_mut().for_each(|(mut text, marker)| {
        let fname_impl = |name, category| -> String {
            if category {
                format!("{:25} ", name)
            } else {
                let s = if marker.0 == game_option_menu_data.selection {
                    ">>"
                } else {
                    ""
                };
                format!("{:>4} {:20} ", s, name)
            }
        };
        let fname_cat = |name| -> String { fname_impl(name, true) };
        let fname_opt = |name| -> String { fname_impl(name, false) };
        let fopt = |name: String, left_arrow, right_arrow| -> String {
            let l = if left_arrow { "<" } else { "" };
            let r = if right_arrow { ">" } else { "" };
            format!("{:1} {:18} {:1}", l, name, r)
        };
        let fopt_n = || -> String { fopt("".to_owned(), false, false) };
        match marker.0 {
            GameOptionMenuSelection::Tetris => {
                text.sections[0].value = fname_opt("TETRIS");
                text.sections[1].value = fopt_n();
            }
            GameOptionMenuSelection::BlankLine0 => {
                text.sections[0].value = fname_cat("");
                text.sections[1].value = fopt_n();
            }
            GameOptionMenuSelection::GameOptionsCategory => {
                text.sections[0].value = fname_cat("$ GAME OPTIONS");
                text.sections[1].value = fopt_n();
            }
            GameOptionMenuSelection::BlankLine1 => {
                text.sections[0].value = fname_cat("");
                text.sections[1].value = fopt_n();
            }
            GameOptionMenuSelection::Transition => {
                text.sections[0].value = fname_opt("TRANSITION");
                text.sections[1].value = fopt(
                    game_config.transition.to_string(),
                    game_config.transition.enum_has_prev(),
                    game_config.transition.enum_has_next(),
                );
            }
            GameOptionMenuSelection::Linecap => {
                text.sections[0].value = fname_opt("LINECAP");
                text.sections[1].value = fopt(
                    game_config.linecap.to_string(),
                    game_config.linecap.enum_has_prev(),
                    game_config.linecap.enum_has_next(),
                );
            }
            GameOptionMenuSelection::Gravity => {
                text.sections[0].value = fname_opt("GRAVITY");
                text.sections[1].value = fopt(
                    game_config.gravity.to_string(),
                    game_config.gravity.enum_has_prev(),
                    game_config.gravity.enum_has_next(),
                );
            }
            GameOptionMenuSelection::Seed => {
                text.sections[0].value = fname_opt("SEED");
                text.sections[1].value = fopt(
                    game_config.seed.to_string(),
                    game_config.seed.enum_has_prev(),
                    game_config.seed.enum_has_next(),
                );
            }
            GameOptionMenuSelection::Scoring => {
                text.sections[0].value = fname_opt("SCORING");
                text.sections[1].value = fopt(
                    game_config.scoring.to_string(),
                    game_config.scoring.enum_has_prev(),
                    game_config.scoring.enum_has_next(),
                );
            }
            GameOptionMenuSelection::TVSystem => {
                text.sections[0].value = fname_opt("TV SYSTEM");
                text.sections[1].value = fopt(
                    game_config.tv_system.to_string(),
                    game_config.tv_system.enum_has_prev(),
                    game_config.tv_system.enum_has_next(),
                );
            }
            GameOptionMenuSelection::NextPieceHint => {
                text.sections[0].value = fname_opt("NEXT PIECE HINT");
                text.sections[1].value = fopt(
                    game_config.next_piece_hint.to_string(),
                    game_config.next_piece_hint.enum_has_prev(),
                    game_config.next_piece_hint.enum_has_next(),
                );
            }
            GameOptionMenuSelection::DASCounter => {
                text.sections[0].value = fname_opt("DAS COUNTER");
                text.sections[1].value = fopt(
                    game_config.das_counter.to_string(),
                    game_config.das_counter.enum_has_prev(),
                    game_config.das_counter.enum_has_next(),
                );
            }
            GameOptionMenuSelection::ControllerMapping => {
                text.sections[0].value = fname_opt("CONTROLLER MAPPING");
                text.sections[1].value = fopt(
                    controller_mapping.to_string(),
                    controller_mapping.enum_has_prev(),
                    controller_mapping.enum_has_next(),
                );
            }
            GameOptionMenuSelection::BlankLine2 => {
                text.sections[0].value = fname_cat("");
                text.sections[1].value = fopt_n();
            }
            GameOptionMenuSelection::VideoOptionsCategory => {
                text.sections[0].value = fname_cat("$ VIDEO OPTIONS");
                text.sections[1].value = fopt_n();
            }
            GameOptionMenuSelection::BlankLine3 => {
                text.sections[0].value = fname_cat("");
                text.sections[1].value = fopt_n();
            }
            GameOptionMenuSelection::ScaleFactor => {
                text.sections[0].value = fname_opt("SCALE FACTOR");
                text.sections[1].value = fopt(
                    scale_factor.to_string(),
                    scale_factor.enum_has_prev(),
                    scale_factor.enum_has_next(),
                );
            }
            #[cfg(not(target_arch = "wasm32"))]
            GameOptionMenuSelection::FPSLimiter => {
                text.sections[0].value = fname_opt("FPS LIMITER");
                text.sections[1].value = fopt(
                    game_option_menu_data.fps_limiter.to_string(),
                    game_option_menu_data.fps_limiter.enum_has_prev(),
                    game_option_menu_data.fps_limiter.enum_has_next(),
                );
            }
            #[cfg(not(target_arch = "wasm32"))]
            GameOptionMenuSelection::WindowMode => {
                text.sections[0].value = fname_opt("WINDOW MODE");
                text.sections[1].value = fopt(
                    game_option_menu_data.window_mode.to_string(),
                    game_option_menu_data.window_mode.enum_has_prev(),
                    game_option_menu_data.window_mode.enum_has_next(),
                );
            }
        }
    });
}

fn handle_input_system(
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<GamepadButton>>,
    controller: Res<Controller>,
    mut controller_mapping: ResMut<ControllerMapping>,
    mut game_option_menu_data: ResMut<GameOptionMenuData>,
    mut game_config: ResMut<GameConfig>,
    mut app_state: ResMut<NextState<AppState>>,
    mut play_sound: EventWriter<PlaySoundEvent>,
    mut scale_factor: ResMut<ScaleFactor>,
    #[cfg(not(target_arch = "wasm32"))] mut framepace_settins: ResMut<
        bevy_framepace::FramepaceSettings,
    >,
    #[cfg(not(target_arch = "wasm32"))] mut query: Query<&mut Window>,
) {
    let player_inputs = PlayerInputs::with_keyboard(&keys)
        | PlayerInputs::with_gamepads(&buttons, &controller, *controller_mapping);

    if player_inputs.soft_reset {
        play_sound.send(PlaySoundEvent::StartGame);
        app_state.set(AppState::Splash);
        return;
    }

    if player_inputs.b.just_pressed {
        app_state.set(AppState::Splash);
        play_sound.send(PlaySoundEvent::StartGame);
        return;
    }

    let mut selection_changed = false;
    let mut option_changed = false;
    let mut scale_changed = false;
    #[cfg(not(target_arch = "wasm32"))]
    let mut fps_changed = false;
    #[cfg(not(target_arch = "wasm32"))]
    let mut window_mode_changed = false;

    match game_option_menu_data.selection {
        GameOptionMenuSelection::Tetris => {
            if player_inputs.up.just_pressed {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    game_option_menu_data.selection = GameOptionMenuSelection::WindowMode;
                }
                #[cfg(target_arch = "wasm32")]
                {
                    game_option_menu_data.selection = GameOptionMenuSelection::ScaleFactor;
                }
                selection_changed = true;
            } else if player_inputs.down.just_pressed {
                game_option_menu_data.selection = GameOptionMenuSelection::Transition;
                selection_changed = true;
            }
            if player_inputs.start.just_pressed {
                play_sound.send(PlaySoundEvent::StartGame);
                app_state.set(AppState::LevelMenu);
            }
        }
        GameOptionMenuSelection::BlankLine0 => (),
        GameOptionMenuSelection::GameOptionsCategory => (),
        GameOptionMenuSelection::BlankLine1 => (),
        GameOptionMenuSelection::Transition => {
            if player_inputs.up.just_pressed {
                game_option_menu_data.selection = GameOptionMenuSelection::Tetris;
                selection_changed = true;
            } else if player_inputs.down.just_pressed {
                game_option_menu_data.selection = GameOptionMenuSelection::Linecap;
                selection_changed = true;
            }

            if player_inputs.right.just_pressed {
                if game_config.transition.enum_next() {
                    option_changed = true;
                }
            } else if player_inputs.left.just_pressed {
                if game_config.transition.enum_prev() {
                    option_changed = true;
                }
            }
        }
        GameOptionMenuSelection::Linecap => {
            if player_inputs.up.just_pressed {
                game_option_menu_data.selection = GameOptionMenuSelection::Transition;
                selection_changed = true;
            } else if player_inputs.down.just_pressed {
                game_option_menu_data.selection = GameOptionMenuSelection::Gravity;
                selection_changed = true;
            }

            if player_inputs.right.just_pressed {
                if game_config.linecap.enum_next() {
                    option_changed = true;
                }
            } else if player_inputs.left.just_pressed {
                if game_config.linecap.enum_prev() {
                    option_changed = true;
                }
            }
        }
        GameOptionMenuSelection::Gravity => {
            if player_inputs.up.just_pressed {
                game_option_menu_data.selection = GameOptionMenuSelection::Linecap;
                selection_changed = true;
            } else if player_inputs.down.just_pressed {
                game_option_menu_data.selection = GameOptionMenuSelection::Seed;
                selection_changed = true;
            }

            if player_inputs.right.just_pressed {
                if game_config.gravity.enum_next() {
                    option_changed = true;
                }
            } else if player_inputs.left.just_pressed {
                if game_config.gravity.enum_prev() {
                    option_changed = true;
                }
            }
        }
        GameOptionMenuSelection::Seed => {
            if player_inputs.up.just_pressed {
                game_option_menu_data.selection = GameOptionMenuSelection::Gravity;
                selection_changed = true;
            } else if player_inputs.down.just_pressed {
                game_option_menu_data.selection = GameOptionMenuSelection::Scoring;
                selection_changed = true;
            }

            if player_inputs.right.just_pressed {
                if game_config.seed.enum_next() {
                    option_changed = true;
                }
            } else if player_inputs.left.just_pressed {
                if game_config.seed.enum_prev() {
                    option_changed = true;
                }
            }
        }
        GameOptionMenuSelection::Scoring => {
            if player_inputs.up.just_pressed {
                game_option_menu_data.selection = GameOptionMenuSelection::Seed;
                selection_changed = true;
            } else if player_inputs.down.just_pressed {
                game_option_menu_data.selection = GameOptionMenuSelection::TVSystem;
                selection_changed = true;
            }

            if player_inputs.right.just_pressed {
                if game_config.scoring.enum_next() {
                    option_changed = true;
                }
            } else if player_inputs.left.just_pressed {
                if game_config.scoring.enum_prev() {
                    option_changed = true;
                }
            }
        }
        GameOptionMenuSelection::TVSystem => {
            if player_inputs.up.just_pressed {
                game_option_menu_data.selection = GameOptionMenuSelection::Scoring;
                selection_changed = true;
            } else if player_inputs.down.just_pressed {
                game_option_menu_data.selection = GameOptionMenuSelection::NextPieceHint;
                selection_changed = true;
            }

            if player_inputs.right.just_pressed {
                if game_config.tv_system.enum_next() {
                    option_changed = true;
                }
            } else if player_inputs.left.just_pressed {
                if game_config.tv_system.enum_prev() {
                    option_changed = true;
                }
            }
        }
        GameOptionMenuSelection::NextPieceHint => {
            if player_inputs.up.just_pressed {
                game_option_menu_data.selection = GameOptionMenuSelection::TVSystem;
                selection_changed = true;
            } else if player_inputs.down.just_pressed {
                game_option_menu_data.selection = GameOptionMenuSelection::DASCounter;
                selection_changed = true;
            }

            if player_inputs.right.just_pressed {
                if game_config.next_piece_hint.enum_next() {
                    option_changed = true;
                }
            } else if player_inputs.left.just_pressed {
                if game_config.next_piece_hint.enum_prev() {
                    option_changed = true;
                }
            }
        }
        GameOptionMenuSelection::DASCounter => {
            if player_inputs.up.just_pressed {
                game_option_menu_data.selection = GameOptionMenuSelection::NextPieceHint;
                selection_changed = true;
            } else if player_inputs.down.just_pressed {
                game_option_menu_data.selection = GameOptionMenuSelection::ControllerMapping;
                selection_changed = true;
            }

            if player_inputs.right.just_pressed {
                if game_config.das_counter.enum_next() {
                    option_changed = true;
                }
            } else if player_inputs.left.just_pressed {
                if game_config.das_counter.enum_prev() {
                    option_changed = true;
                }
            }
        }
        GameOptionMenuSelection::ControllerMapping => {
            if player_inputs.up.just_pressed {
                game_option_menu_data.selection = GameOptionMenuSelection::DASCounter;
                selection_changed = true;
            } else if player_inputs.down.just_pressed {
                game_option_menu_data.selection = GameOptionMenuSelection::ScaleFactor;
                selection_changed = true;
            }

            if player_inputs.right.just_pressed {
                if controller_mapping.enum_next() {
                    option_changed = true;
                }
            } else if player_inputs.left.just_pressed {
                if controller_mapping.enum_prev() {
                    option_changed = true;
                }
            }
        }
        GameOptionMenuSelection::BlankLine2 => (),
        GameOptionMenuSelection::VideoOptionsCategory => (),
        GameOptionMenuSelection::BlankLine3 => (),
        GameOptionMenuSelection::ScaleFactor => {
            if player_inputs.up.just_pressed {
                game_option_menu_data.selection = GameOptionMenuSelection::ControllerMapping;
                selection_changed = true;
            } else if player_inputs.down.just_pressed {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    game_option_menu_data.selection = GameOptionMenuSelection::FPSLimiter;
                }
                #[cfg(target_arch = "wasm32")]
                {
                    game_option_menu_data.selection = GameOptionMenuSelection::Tetris;
                }
                selection_changed = true;
            }

            if player_inputs.right.just_pressed {
                if scale_factor.enum_next() {
                    scale_changed = true;
                }
            } else if player_inputs.left.just_pressed {
                if scale_factor.enum_prev() {
                    scale_changed = true;
                }
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        GameOptionMenuSelection::FPSLimiter => {
            if player_inputs.up.just_pressed {
                game_option_menu_data.selection = GameOptionMenuSelection::ScaleFactor;
                selection_changed = true;
            } else if player_inputs.down.just_pressed {
                game_option_menu_data.selection = GameOptionMenuSelection::WindowMode;
                selection_changed = true;
            }

            if player_inputs.right.just_pressed {
                if game_option_menu_data.fps_limiter.enum_next() {
                    fps_changed = true;
                }
            } else if player_inputs.left.just_pressed {
                if game_option_menu_data.fps_limiter.enum_prev() {
                    fps_changed = true;
                }
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        GameOptionMenuSelection::WindowMode => {
            if player_inputs.up.just_pressed {
                game_option_menu_data.selection = GameOptionMenuSelection::FPSLimiter;
                selection_changed = true;
            } else if player_inputs.down.just_pressed {
                game_option_menu_data.selection = GameOptionMenuSelection::Tetris;
                selection_changed = true;
            }

            if player_inputs.right.just_pressed {
                if game_option_menu_data.window_mode.enum_next() {
                    window_mode_changed = true;
                }
            } else if player_inputs.left.just_pressed {
                if game_option_menu_data.window_mode.enum_prev() {
                    window_mode_changed = true;
                }
            }
        }
    }

    if scale_changed {
        app_state.set(AppState::ChangeScale);
    }
    option_changed |= scale_changed;
    #[cfg(not(target_arch = "wasm32"))]
    {
        if fps_changed {
            *framepace_settins = bevy_framepace::FramepaceSettings {
                limiter: game_option_menu_data.fps_limiter.get_limiter(),
            };
        }
        option_changed |= fps_changed;
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        if window_mode_changed {
            let mut window = query.single_mut();
            window.mode = game_option_menu_data.window_mode.get_window_mode();
        }
        option_changed |= window_mode_changed;
    }
    if selection_changed || option_changed {
        play_sound.send(PlaySoundEvent::MoveCursor);
    }
}
