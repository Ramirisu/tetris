use bevy::{color::palettes::css::WHITE, prelude::*};

use crate::{
    app_state::AppState,
    audio::plugin::PlaySoundEvent,
    controller::Controller,
    game::{
        das_indicator::DASIndicator, drop_speed::DropSpeed, game::GameConfig, linecap::Linecap,
        next_piece_hint::NextPieceHint, transition::Transition,
    },
    inputs::{ControllerMapping, PlayerInputs},
    level_menu::plugin::LevelMenuData,
    logo::{load_logo_images, TETRIS_BITMAP},
    scale::plugin::ScaleFactor,
    utility::despawn_all,
};

#[cfg(not(target_arch = "wasm32"))]
use bevy::window::WindowMode;

use super::{fps_limiter::FPSLimiter, transform::GameOptionMenuTransform};

pub fn setup(app: &mut App) {
    app.add_plugins(bevy_framepace::FramepacePlugin)
        .add_systems(Startup, init_framepace_settings)
        .insert_resource(GameOptionMenuTransform::default())
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
    DropSpeed,
    NextPieceHint,
    DASIndicator,
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
        type ArrayType = [GameOptionMenuSelection; 16];
        #[cfg(target_arch = "wasm32")]
        type ArrayType = [GameOptionMenuSelection; 14];
        const STATES: ArrayType = [
            GameOptionMenuSelection::Tetris,
            GameOptionMenuSelection::BlankLine0,
            GameOptionMenuSelection::GameOptionsCategory,
            GameOptionMenuSelection::BlankLine1,
            GameOptionMenuSelection::Transition,
            GameOptionMenuSelection::Linecap,
            GameOptionMenuSelection::DropSpeed,
            GameOptionMenuSelection::NextPieceHint,
            GameOptionMenuSelection::DASIndicator,
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
    game_config: GameConfig,
    #[cfg(not(target_arch = "wasm32"))]
    fps_limiter: FPSLimiter,
    #[cfg(not(target_arch = "wasm32"))]
    window_mode: WindowMode,
}

impl GameOptionMenuData {
    pub fn new() -> Self {
        Self {
            selection: GameOptionMenuSelection::default(),
            game_config: GameConfig::default(),
            #[cfg(not(target_arch = "wasm32"))]
            fps_limiter: FPSLimiter::default(),
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
    controller_mapping: Res<ControllerMapping>,
    scale_factor: Res<ScaleFactor>,
    app_state: Res<State<AppState>>,
) {
    if !app_state.is_changed()
        && !game_option_menu_data.is_changed()
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
        let fopt_impl = |name, left_arrow, right_arrow| -> String {
            let l = if left_arrow { "<" } else { "" };
            let r = if right_arrow { ">" } else { "" };
            format!("{:1} {:12} {:1}", l, name, r)
        };
        let fopt_n = |name| -> String { fopt_impl(name, false, false) };
        let fopt_l = |name| -> String { fopt_impl(name, true, false) };
        let fopt_m = |name| -> String { fopt_impl(name, true, true) };
        let fopt_r = |name| -> String { fopt_impl(name, false, true) };
        match marker.0 {
            GameOptionMenuSelection::Tetris => {
                text.sections[0].value = fname_opt("TETRIS");
                text.sections[1].value = fopt_n("");
            }
            GameOptionMenuSelection::BlankLine0 => {
                text.sections[0].value = fname_cat("");
                text.sections[1].value = fopt_n("");
            }
            GameOptionMenuSelection::GameOptionsCategory => {
                text.sections[0].value = fname_cat("$ GAME OPTIONS");
                text.sections[1].value = fopt_n("");
            }
            GameOptionMenuSelection::BlankLine1 => {
                text.sections[0].value = fname_cat("");
                text.sections[1].value = fopt_n("");
            }
            GameOptionMenuSelection::Transition => {
                text.sections[0].value = fname_opt("TRANSITION");
                match game_option_menu_data.game_config.transition {
                    Transition::Classic => text.sections[1].value = fopt_r("CLASSIC"),
                    Transition::Fxied => text.sections[1].value = fopt_m("FIXED"),
                    Transition::Every10Lines => text.sections[1].value = fopt_m("10 LINES"),
                    Transition::Every4Lines => text.sections[1].value = fopt_l(" 4 LINES"),
                };
            }
            GameOptionMenuSelection::Linecap => {
                text.sections[0].value = fname_opt("LINECAP");
                match game_option_menu_data.game_config.linecap {
                    Linecap::None => text.sections[1].value = fopt_r("OFF"),
                    Linecap::KillScreenX2 => text.sections[1].value = fopt_l("ON"),
                }
            }
            GameOptionMenuSelection::DropSpeed => {
                text.sections[0].value = fname_opt("DROPSPEED");
                match game_option_menu_data.game_config.drop_speed {
                    DropSpeed::Level => text.sections[1].value = fopt_r("LEVEL"),
                    DropSpeed::Locked => text.sections[1].value = fopt_l("LOCKED"),
                };
            }
            GameOptionMenuSelection::NextPieceHint => {
                text.sections[0].value = fname_opt("NEXT PIECE HINT");
                match game_option_menu_data.game_config.next_piece_hint {
                    NextPieceHint::Off => text.sections[1].value = fopt_r("OFF"),
                    NextPieceHint::Classic => text.sections[1].value = fopt_m("CLASSIC"),
                    NextPieceHint::Modern => text.sections[1].value = fopt_l("MODERN"),
                }
            }
            GameOptionMenuSelection::DASIndicator => {
                text.sections[0].value = fname_opt("DAS INDICATOR");
                match game_option_menu_data.game_config.das_indicator {
                    DASIndicator::Off => text.sections[1].value = fopt_r("OFF"),
                    DASIndicator::On => text.sections[1].value = fopt_l("ON"),
                }
            }
            GameOptionMenuSelection::ControllerMapping => {
                text.sections[0].value = fname_opt("CONTROLLER MAPPING");
                match *controller_mapping {
                    ControllerMapping::MappingA => text.sections[1].value = fopt_r("MAPPING A"),
                    ControllerMapping::MappingB => text.sections[1].value = fopt_l("MAPPING B"),
                };
            }
            GameOptionMenuSelection::BlankLine2 => {
                text.sections[0].value = fname_cat("");
                text.sections[1].value = fopt_n("");
            }
            GameOptionMenuSelection::VideoOptionsCategory => {
                text.sections[0].value = fname_cat("$ VIDEO OPTIONS");
                text.sections[1].value = fopt_n("");
            }
            GameOptionMenuSelection::BlankLine3 => {
                text.sections[0].value = fname_cat("");
                text.sections[1].value = fopt_n("");
            }
            GameOptionMenuSelection::ScaleFactor => {
                text.sections[0].value = fname_opt("SCALE FACTOR");
                match *scale_factor {
                    ScaleFactor::S720 => text.sections[1].value = fopt_r("0.66 (720P)"),
                    ScaleFactor::S1080 => text.sections[1].value = fopt_m("1.00 (1080P)"),
                    ScaleFactor::S1440 => text.sections[1].value = fopt_m("1.33 (1440P)"),
                    ScaleFactor::S1800 => text.sections[1].value = fopt_m("1.66 (1800P)"),
                    ScaleFactor::S2160 => text.sections[1].value = fopt_m("2.00 (2160P)"),
                    ScaleFactor::S3240 => text.sections[1].value = fopt_m("3.00 (3240P)"),
                    ScaleFactor::S4320 => text.sections[1].value = fopt_l("4.00 (4320P)"),
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            GameOptionMenuSelection::FPSLimiter => {
                text.sections[0].value = fname_opt("FPS LIMITER");
                match game_option_menu_data.fps_limiter {
                    FPSLimiter::Auto => text.sections[1].value = fopt_r("AUTO"),
                    FPSLimiter::Unlimited => text.sections[1].value = fopt_m("UNLIMITED"),
                    FPSLimiter::F60 => text.sections[1].value = fopt_m("60 FPS"),
                    FPSLimiter::F144 => text.sections[1].value = fopt_m("144 FPS"),
                    FPSLimiter::F240 => text.sections[1].value = fopt_m("240 FPS"),
                    FPSLimiter::F360 => text.sections[1].value = fopt_m("360 FPS"),
                    FPSLimiter::F480 => text.sections[1].value = fopt_l("480 FPS"),
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            GameOptionMenuSelection::WindowMode => {
                text.sections[0].value = fname_opt("WINDOW MODE");
                match game_option_menu_data.window_mode {
                    WindowMode::Windowed => {
                        text.sections[1].value = fopt_r("WINDOWED");
                    }
                    WindowMode::BorderlessFullscreen => {
                        text.sections[1].value = fopt_m("BORDERLESS");
                    }
                    WindowMode::Fullscreen => {
                        text.sections[1].value = fopt_l("FULLSCREEN");
                    }
                    _ => (),
                };
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
    mut level_menu_data: ResMut<LevelMenuData>,
    mut app_state: ResMut<NextState<AppState>>,
    mut e_play_sound: EventWriter<PlaySoundEvent>,
    mut scale_factor: ResMut<ScaleFactor>,
    #[cfg(not(target_arch = "wasm32"))] mut framepace_settins: ResMut<
        bevy_framepace::FramepaceSettings,
    >,
    #[cfg(not(target_arch = "wasm32"))] mut query: Query<&mut Window>,
) {
    let player_inputs = PlayerInputs::with_keyboard(&keys)
        | PlayerInputs::with_gamepads(&buttons, &controller, *controller_mapping);

    if player_inputs.soft_reset {
        e_play_sound.send(PlaySoundEvent::StartGame);
        app_state.set(AppState::Splash);
        return;
    }

    if player_inputs.b.0 {
        app_state.set(AppState::Splash);
        e_play_sound.send(PlaySoundEvent::StartGame);
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
            if player_inputs.up.0 {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    game_option_menu_data.selection = GameOptionMenuSelection::WindowMode;
                }
                #[cfg(target_arch = "wasm32")]
                {
                    game_option_menu_data.selection = GameOptionMenuSelection::ScaleFactor;
                }
                selection_changed = true;
            } else if player_inputs.down.0 {
                game_option_menu_data.selection = GameOptionMenuSelection::Transition;
                selection_changed = true;
            }
            if player_inputs.start {
                level_menu_data.game_config = game_option_menu_data.game_config;
                e_play_sound.send(PlaySoundEvent::StartGame);
                app_state.set(AppState::LevelMenu);
            }
        }
        GameOptionMenuSelection::BlankLine0 => (),
        GameOptionMenuSelection::GameOptionsCategory => (),
        GameOptionMenuSelection::BlankLine1 => (),
        GameOptionMenuSelection::Transition => {
            if player_inputs.up.0 {
                game_option_menu_data.selection = GameOptionMenuSelection::Tetris;
                selection_changed = true;
            } else if player_inputs.down.0 {
                game_option_menu_data.selection = GameOptionMenuSelection::Linecap;
                selection_changed = true;
            }

            if player_inputs.right.0 {
                if let Some(_) = game_option_menu_data.game_config.transition.enum_next() {
                    option_changed = true;
                }
            } else if player_inputs.left.0 {
                if let Some(_) = game_option_menu_data.game_config.transition.enum_prev() {
                    option_changed = true;
                }
            }
        }
        GameOptionMenuSelection::Linecap => {
            if player_inputs.up.0 {
                game_option_menu_data.selection = GameOptionMenuSelection::Transition;
                selection_changed = true;
            } else if player_inputs.down.0 {
                game_option_menu_data.selection = GameOptionMenuSelection::DropSpeed;
                selection_changed = true;
            }

            if player_inputs.right.0 {
                if let Some(_) = game_option_menu_data.game_config.linecap.enum_next() {
                    option_changed = true;
                }
            } else if player_inputs.left.0 {
                if let Some(_) = game_option_menu_data.game_config.linecap.enum_prev() {
                    option_changed = true;
                }
            }
        }
        GameOptionMenuSelection::DropSpeed => {
            if player_inputs.up.0 {
                game_option_menu_data.selection = GameOptionMenuSelection::Linecap;
                selection_changed = true;
            } else if player_inputs.down.0 {
                game_option_menu_data.selection = GameOptionMenuSelection::NextPieceHint;
                selection_changed = true;
            }

            if player_inputs.right.0 {
                if let Some(_) = game_option_menu_data.game_config.drop_speed.enum_next() {
                    option_changed = true;
                }
            } else if player_inputs.left.0 {
                if let Some(_) = game_option_menu_data.game_config.drop_speed.enum_prev() {
                    option_changed = true;
                }
            }
        }
        GameOptionMenuSelection::NextPieceHint => {
            if player_inputs.up.0 {
                game_option_menu_data.selection = GameOptionMenuSelection::DropSpeed;
                selection_changed = true;
            } else if player_inputs.down.0 {
                game_option_menu_data.selection = GameOptionMenuSelection::DASIndicator;
                selection_changed = true;
            }

            if player_inputs.right.0 {
                if let Some(_) = game_option_menu_data
                    .game_config
                    .next_piece_hint
                    .enum_next()
                {
                    option_changed = true;
                }
            } else if player_inputs.left.0 {
                if let Some(_) = game_option_menu_data
                    .game_config
                    .next_piece_hint
                    .enum_prev()
                {
                    option_changed = true;
                }
            }
        }
        GameOptionMenuSelection::DASIndicator => {
            if player_inputs.up.0 {
                game_option_menu_data.selection = GameOptionMenuSelection::NextPieceHint;
                selection_changed = true;
            } else if player_inputs.down.0 {
                game_option_menu_data.selection = GameOptionMenuSelection::ControllerMapping;
                selection_changed = true;
            }

            if player_inputs.right.0 {
                if let Some(_) = game_option_menu_data.game_config.das_indicator.enum_next() {
                    option_changed = true;
                }
            } else if player_inputs.left.0 {
                if let Some(_) = game_option_menu_data.game_config.das_indicator.enum_prev() {
                    option_changed = true;
                }
            }
        }
        GameOptionMenuSelection::ControllerMapping => {
            if player_inputs.up.0 {
                game_option_menu_data.selection = GameOptionMenuSelection::DASIndicator;
                selection_changed = true;
            } else if player_inputs.down.0 {
                game_option_menu_data.selection = GameOptionMenuSelection::ScaleFactor;
                selection_changed = true;
            }

            if player_inputs.right.0 {
                if let Some(_) = controller_mapping.enum_next() {
                    option_changed = true;
                }
            } else if player_inputs.left.0 {
                if let Some(_) = controller_mapping.enum_prev() {
                    option_changed = true;
                }
            }
        }
        GameOptionMenuSelection::BlankLine2 => (),
        GameOptionMenuSelection::VideoOptionsCategory => (),
        GameOptionMenuSelection::BlankLine3 => (),
        GameOptionMenuSelection::ScaleFactor => {
            if player_inputs.up.0 {
                game_option_menu_data.selection = GameOptionMenuSelection::ControllerMapping;
                selection_changed = true;
            } else if player_inputs.down.0 {
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

            if player_inputs.right.0 {
                if let Some(_) = scale_factor.enum_next() {
                    scale_changed = true;
                }
            } else if player_inputs.left.0 {
                if let Some(_) = scale_factor.enum_prev() {
                    scale_changed = true;
                }
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        GameOptionMenuSelection::FPSLimiter => {
            if player_inputs.up.0 {
                game_option_menu_data.selection = GameOptionMenuSelection::ScaleFactor;
                selection_changed = true;
            } else if player_inputs.down.0 {
                game_option_menu_data.selection = GameOptionMenuSelection::WindowMode;
                selection_changed = true;
            }

            if player_inputs.right.0 {
                if let Some(_) = game_option_menu_data.fps_limiter.enum_next() {
                    fps_changed = true;
                }
            } else if player_inputs.left.0 {
                if let Some(_) = game_option_menu_data.fps_limiter.enum_prev() {
                    fps_changed = true;
                }
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        GameOptionMenuSelection::WindowMode => {
            if player_inputs.up.0 {
                game_option_menu_data.selection = GameOptionMenuSelection::FPSLimiter;
                selection_changed = true;
            } else if player_inputs.down.0 {
                game_option_menu_data.selection = GameOptionMenuSelection::Tetris;
                selection_changed = true;
            }

            match game_option_menu_data.window_mode {
                WindowMode::Windowed => {
                    if player_inputs.right.0 {
                        game_option_menu_data.window_mode = WindowMode::BorderlessFullscreen;
                        window_mode_changed = true;
                    }
                }
                WindowMode::BorderlessFullscreen => {
                    if player_inputs.right.0 {
                        game_option_menu_data.window_mode = WindowMode::Fullscreen;
                        window_mode_changed = true;
                    } else if player_inputs.left.0 {
                        game_option_menu_data.window_mode = WindowMode::Windowed;
                        window_mode_changed = true;
                    }
                }
                WindowMode::Fullscreen => {
                    if player_inputs.left.0 {
                        game_option_menu_data.window_mode = WindowMode::BorderlessFullscreen;
                        window_mode_changed = true;
                    }
                }
                _ => (),
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
            window.mode = game_option_menu_data.window_mode;
        }
        option_changed |= window_mode_changed;
    }
    if selection_changed || option_changed {
        e_play_sound.send(PlaySoundEvent::MoveCursor);
    }
}
