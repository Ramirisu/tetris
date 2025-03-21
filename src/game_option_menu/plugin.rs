use bevy::{color::palettes::css::WHITE, prelude::*};
use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, FromRepr};

use crate::{
    app_state::AppState,
    audio::plugin::PlaySoundEvent,
    enum_advance, enum_advance_cycle,
    game::{
        game::GameConfig,
        seed::{SEED_BYTES_USED, Seed},
        seeding::Seeding,
    },
    input::{controller_mapping::ControllerMapping, player_inputs::PlayerInputs},
    logo::{TETRIS_BITMAP, load_logo_images},
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
        limiter: FPSLimiter::default().into(),
    };
}

#[derive(Component)]
struct GameOptionEntityMarker(pub GameOptionMenuSelection);

#[derive(Component)]
struct GameOptionMenuEntityMarker;

#[derive(Default, Clone, Copy, PartialEq, Eq, FromRepr, EnumIter, EnumCount)]
enum GameOptionMenuSelection {
    #[default]
    Tetris,
    Transition,
    Linecap,
    Gravity,
    Seeding,
    Seed,
    Scoring,
    TVSystem,
    NextPieceHint,
    Invisible,
    DASCounter,
    ControllerMapping,
    ScaleFactor,
    #[cfg(not(target_arch = "wasm32"))]
    FPSLimiter,
    #[cfg(not(target_arch = "wasm32"))]
    WindowMode,
}

enum_advance::enum_advance_derive!(GameOptionMenuSelection);
enum_advance_cycle::enum_advance_cycle_derive!(GameOptionMenuSelection);

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum GameOptionMenuSeedSelection {
    #[default]
    None,
    Index(usize),
}

const GAME_OPTION_MENU_SEED_FIRST: usize = 0;
const GAME_OPTION_MENU_SEED_LAST: usize = SEED_BYTES_USED * 2 - 1;

#[derive(Resource)]
struct GameOptionMenuData {
    selection: GameOptionMenuSelection,
    seed_selection: GameOptionMenuSeedSelection,
    #[cfg(not(target_arch = "wasm32"))]
    fps_limiter: FPSLimiter,
    #[cfg(not(target_arch = "wasm32"))]
    window_mode: WindowMode,
}

impl GameOptionMenuData {
    pub fn new() -> Self {
        Self {
            selection: GameOptionMenuSelection::default(),
            seed_selection: GameOptionMenuSeedSelection::default(),
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
    transform: Res<GameOptionMenuTransform>,
) {
    let logo_images = load_logo_images(&mut image_assets);

    commands
        .spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            GameOptionMenuEntityMarker,
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    display: Display::Grid,
                    grid_template_columns: vec![GridTrack::auto(); TETRIS_BITMAP[0].len()],
                    margin: UiRect::all(Val::Px(transform.fs_medium())),
                    ..default()
                })
                .with_children(|parent| {
                    TETRIS_BITMAP.iter().for_each(|rows| {
                        rows.iter().for_each(|sqr| {
                            parent.spawn((
                                Node {
                                    width: Val::Px(transform.fs_small()),
                                    height: Val::Px(transform.fs_small()),
                                    ..default()
                                },
                                ImageNode::new(logo_images[(*sqr) as usize].clone()),
                            ));
                        })
                    });
                });

            parent
                .spawn(Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    margin: UiRect::all(Val::Px(transform.fs_medium())),
                    ..default()
                })
                .with_children(|parent| {
                    let mut selection = GameOptionMenuSelection::default();
                    loop {
                        parent
                            .spawn((
                                Text::default(),
                                TextFont::from_font_size(transform.fs_medium()),
                                TextColor::from(WHITE),
                                GameOptionEntityMarker(selection),
                            ))
                            .with_child((
                                TextSpan::default(),
                                TextFont::from_font_size(transform.fs_medium()),
                                TextColor::from(WHITE),
                            ));

                        if let Some(e) = selection.enum_next() {
                            selection = e;
                        } else {
                            break;
                        }
                    }
                });
        });
}

fn update_ui_system(
    mut query: Query<(Entity, &GameOptionEntityMarker)>,
    mut tw: TextUiWriter,
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

    query.iter_mut().for_each(|(entity, marker)| {
        let fname = |name| -> String {
            let s = if marker.0 == game_option_menu_data.selection {
                ">>"
            } else {
                ""
            };
            format!("{:>2} {:20} ", s, name)
        };
        let fopt = |name: String, left_arrow, right_arrow| -> String {
            let l = if left_arrow { "<" } else { "" };
            let r = if right_arrow { ">" } else { "" };
            format!("{:1} {:18} {:1}", l, name, r)
        };
        let fopt_n = || -> String { fopt("".to_owned(), false, false) };
        match marker.0 {
            GameOptionMenuSelection::Tetris => {
                *tw.text(entity, 0) = fname("TETRIS");
                *tw.text(entity, 1) = fopt_n();
            }
            GameOptionMenuSelection::Transition => {
                *tw.text(entity, 0) = fname("TRANSITION");
                *tw.text(entity, 1) = fopt(
                    game_config.transition.to_string(),
                    game_config.transition.enum_prev().is_some(),
                    game_config.transition.enum_next().is_some(),
                );
            }
            GameOptionMenuSelection::Linecap => {
                *tw.text(entity, 0) = fname("LINECAP");
                *tw.text(entity, 1) = fopt(
                    game_config.linecap.to_string(),
                    game_config.linecap.enum_prev().is_some(),
                    game_config.linecap.enum_next().is_some(),
                );
            }
            GameOptionMenuSelection::Gravity => {
                *tw.text(entity, 0) = fname("GRAVITY");
                *tw.text(entity, 1) = fopt(
                    game_config.gravity.to_string(),
                    game_config.gravity.enum_prev().is_some(),
                    game_config.gravity.enum_next().is_some(),
                );
            }
            GameOptionMenuSelection::Seeding => {
                *tw.text(entity, 0) = fname("SEEDING");
                *tw.text(entity, 1) = fopt(
                    game_config.seeding.to_string(),
                    game_config.seeding.enum_prev().is_some(),
                    game_config.seeding.enum_next().is_some(),
                );
            }
            GameOptionMenuSelection::Seed => {
                *tw.text(entity, 0) = fname("SEED");
                *tw.text(entity, 1) = match game_config.seeding {
                    Seeding::System => fopt_n(),
                    Seeding::Custom => fopt(game_config.seed.to_string(), false, false),
                };
            }
            GameOptionMenuSelection::Scoring => {
                *tw.text(entity, 0) = fname("SCORING");
                *tw.text(entity, 1) = fopt(
                    game_config.scoring.to_string(),
                    game_config.scoring.enum_prev().is_some(),
                    game_config.scoring.enum_next().is_some(),
                );
            }
            GameOptionMenuSelection::TVSystem => {
                *tw.text(entity, 0) = fname("TV SYSTEM");
                *tw.text(entity, 1) = fopt(
                    game_config.tv_system.to_string(),
                    game_config.tv_system.enum_prev().is_some(),
                    game_config.tv_system.enum_next().is_some(),
                );
            }
            GameOptionMenuSelection::NextPieceHint => {
                *tw.text(entity, 0) = fname("NEXT PIECE HINT");
                *tw.text(entity, 1) = fopt(
                    game_config.next_piece_hint.to_string(),
                    game_config.next_piece_hint.enum_prev().is_some(),
                    game_config.next_piece_hint.enum_next().is_some(),
                );
            }
            GameOptionMenuSelection::Invisible => {
                *tw.text(entity, 0) = fname("INVISIBLE");
                *tw.text(entity, 1) = fopt(
                    game_config.invisible.to_string(),
                    game_config.invisible.enum_prev().is_some(),
                    game_config.invisible.enum_next().is_some(),
                );
            }
            GameOptionMenuSelection::DASCounter => {
                *tw.text(entity, 0) = fname("DAS COUNTER");
                *tw.text(entity, 1) = fopt(
                    game_config.das_counter.to_string(),
                    game_config.das_counter.enum_prev().is_some(),
                    game_config.das_counter.enum_next().is_some(),
                );
            }
            GameOptionMenuSelection::ControllerMapping => {
                *tw.text(entity, 0) = fname("CONTROLLER MAPPING");
                *tw.text(entity, 1) = fopt(
                    controller_mapping.to_string(),
                    controller_mapping.enum_prev().is_some(),
                    controller_mapping.enum_next().is_some(),
                );
            }
            GameOptionMenuSelection::ScaleFactor => {
                *tw.text(entity, 0) = fname("SCALE FACTOR");
                *tw.text(entity, 1) = fopt(
                    scale_factor.to_string(),
                    scale_factor.enum_prev().is_some(),
                    scale_factor.enum_next().is_some(),
                );
            }
            #[cfg(not(target_arch = "wasm32"))]
            GameOptionMenuSelection::FPSLimiter => {
                *tw.text(entity, 0) = fname("FPS LIMITER");
                *tw.text(entity, 1) = fopt(
                    game_option_menu_data.fps_limiter.to_string(),
                    game_option_menu_data.fps_limiter.enum_prev().is_some(),
                    game_option_menu_data.fps_limiter.enum_next().is_some(),
                );
            }
            #[cfg(not(target_arch = "wasm32"))]
            GameOptionMenuSelection::WindowMode => {
                *tw.text(entity, 0) = fname("WINDOW MODE");
                *tw.text(entity, 1) = fopt(
                    game_option_menu_data.window_mode.to_string(),
                    game_option_menu_data.window_mode.enum_prev().is_some(),
                    game_option_menu_data.window_mode.enum_next().is_some(),
                );
            }
        }
    });
}

fn handle_input_system(
    keys: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
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
        | PlayerInputs::with_gamepads(gamepads, *controller_mapping);

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

    if game_option_menu_data.selection != GameOptionMenuSelection::Seed
        || game_config.seeding == Seeding::System
        || game_option_menu_data.seed_selection == GameOptionMenuSeedSelection::None
    {
        match (
            player_inputs.up.just_pressed,
            player_inputs.down.just_pressed,
        ) {
            (true, false) => {
                game_option_menu_data.selection = game_option_menu_data.selection.enum_prev_cycle();
                play_sound.send(PlaySoundEvent::MoveCursor);

                return;
            }
            (false, true) => {
                game_option_menu_data.selection = game_option_menu_data.selection.enum_next_cycle();
                play_sound.send(PlaySoundEvent::MoveCursor);
                return;
            }
            _ => (),
        }
    }

    let mut option_changed = false;
    let mut scale_changed = false;
    #[cfg(not(target_arch = "wasm32"))]
    let mut fps_changed = false;
    #[cfg(not(target_arch = "wasm32"))]
    let mut window_mode_changed = false;

    match game_option_menu_data.selection {
        GameOptionMenuSelection::Tetris => {
            if player_inputs.start.just_pressed {
                play_sound.send(PlaySoundEvent::StartGame);
                app_state.set(AppState::LevelMenu);
            }
        }
        GameOptionMenuSelection::Transition => {
            if player_inputs.right.just_pressed {
                if let Some(e) = game_config.transition.enum_next() {
                    game_config.transition = e;
                    option_changed = true;
                }
            } else if player_inputs.left.just_pressed {
                if let Some(e) = game_config.transition.enum_prev() {
                    game_config.transition = e;
                    option_changed = true;
                }
            }
        }
        GameOptionMenuSelection::Linecap => {
            if player_inputs.right.just_pressed {
                if let Some(e) = game_config.linecap.enum_next() {
                    game_config.linecap = e;
                    option_changed = true;
                }
            } else if player_inputs.left.just_pressed {
                if let Some(e) = game_config.linecap.enum_prev() {
                    game_config.linecap = e;
                    option_changed = true;
                }
            }
        }
        GameOptionMenuSelection::Gravity => {
            if player_inputs.right.just_pressed {
                if let Some(e) = game_config.gravity.enum_next() {
                    game_config.gravity = e;
                    option_changed = true;
                }
            } else if player_inputs.left.just_pressed {
                if let Some(e) = game_config.gravity.enum_prev() {
                    game_config.gravity = e;
                    option_changed = true;
                }
            }
        }
        GameOptionMenuSelection::Seeding => {
            if player_inputs.right.just_pressed {
                if let Some(e) = game_config.seeding.enum_next() {
                    game_config.seeding = e;
                    option_changed = true;
                }
            } else if player_inputs.left.just_pressed {
                if let Some(e) = game_config.seeding.enum_prev() {
                    game_config.seeding = e;
                    option_changed = true;
                }
            }
        }
        GameOptionMenuSelection::Seed => {
            if player_inputs.start.just_pressed {
                match game_option_menu_data.seed_selection {
                    GameOptionMenuSeedSelection::None => {
                        game_option_menu_data.seed_selection =
                            GameOptionMenuSeedSelection::Index(GAME_OPTION_MENU_SEED_LAST);
                    }
                    GameOptionMenuSeedSelection::Index(_) => {
                        game_option_menu_data.seed_selection = GameOptionMenuSeedSelection::None;
                    }
                }
                option_changed = true;
            } else if player_inputs.right.just_pressed {
                match game_option_menu_data.seed_selection {
                    GameOptionMenuSeedSelection::Index(GAME_OPTION_MENU_SEED_FIRST) => (),
                    GameOptionMenuSeedSelection::Index(index) => {
                        game_option_menu_data.seed_selection =
                            GameOptionMenuSeedSelection::Index(index - 1);
                        option_changed = true;
                    }
                    GameOptionMenuSeedSelection::None => (),
                }
            } else if player_inputs.left.just_pressed {
                match game_option_menu_data.seed_selection {
                    GameOptionMenuSeedSelection::Index(GAME_OPTION_MENU_SEED_LAST) => (),
                    GameOptionMenuSeedSelection::Index(index) => {
                        game_option_menu_data.seed_selection =
                            GameOptionMenuSeedSelection::Index(index + 1);
                        option_changed = true;
                    }
                    GameOptionMenuSeedSelection::None => (),
                }
            } else if player_inputs.up.just_pressed {
                match game_option_menu_data.seed_selection {
                    GameOptionMenuSeedSelection::Index(index) => game_config.seed.increment(index),
                    GameOptionMenuSeedSelection::None => (),
                }
            } else if player_inputs.down.just_pressed {
                match game_option_menu_data.seed_selection {
                    GameOptionMenuSeedSelection::Index(index) => game_config.seed.decrement(index),
                    GameOptionMenuSeedSelection::None => (),
                }
            } else if player_inputs.select.just_pressed {
                game_config.seed = Seed::new();
            }
        }
        GameOptionMenuSelection::Scoring => {
            if player_inputs.right.just_pressed {
                if let Some(e) = game_config.scoring.enum_next() {
                    game_config.scoring = e;
                    option_changed = true;
                }
            } else if player_inputs.left.just_pressed {
                if let Some(e) = game_config.scoring.enum_prev() {
                    game_config.scoring = e;
                    option_changed = true;
                }
            }
        }
        GameOptionMenuSelection::TVSystem => {
            if player_inputs.right.just_pressed {
                if let Some(e) = game_config.tv_system.enum_next() {
                    game_config.tv_system = e;
                    option_changed = true;
                }
            } else if player_inputs.left.just_pressed {
                if let Some(e) = game_config.tv_system.enum_prev() {
                    game_config.tv_system = e;
                    option_changed = true;
                }
            }
        }
        GameOptionMenuSelection::NextPieceHint => {
            if player_inputs.right.just_pressed {
                if let Some(e) = game_config.next_piece_hint.enum_next() {
                    game_config.next_piece_hint = e;
                    option_changed = true;
                }
            } else if player_inputs.left.just_pressed {
                if let Some(e) = game_config.next_piece_hint.enum_prev() {
                    game_config.next_piece_hint = e;
                    option_changed = true;
                }
            }
        }
        GameOptionMenuSelection::Invisible => {
            if player_inputs.right.just_pressed {
                if let Some(e) = game_config.invisible.enum_next() {
                    game_config.invisible = e;
                    option_changed = true;
                }
            } else if player_inputs.left.just_pressed {
                if let Some(e) = game_config.invisible.enum_prev() {
                    game_config.invisible = e;
                    option_changed = true;
                }
            }
        }
        GameOptionMenuSelection::DASCounter => {
            if player_inputs.right.just_pressed {
                if let Some(e) = game_config.das_counter.enum_next() {
                    game_config.das_counter = e;
                    option_changed = true;
                }
            } else if player_inputs.left.just_pressed {
                if let Some(e) = game_config.das_counter.enum_prev() {
                    game_config.das_counter = e;
                    option_changed = true;
                }
            }
        }
        GameOptionMenuSelection::ControllerMapping => {
            if player_inputs.right.just_pressed {
                if let Some(e) = controller_mapping.enum_next() {
                    *controller_mapping = e;
                    option_changed = true;
                }
            } else if player_inputs.left.just_pressed {
                if let Some(e) = controller_mapping.enum_prev() {
                    *controller_mapping = e;
                    option_changed = true;
                }
            }
        }
        GameOptionMenuSelection::ScaleFactor => {
            if player_inputs.right.just_pressed {
                if let Some(e) = scale_factor.enum_next() {
                    *scale_factor = e;
                    scale_changed = true;
                }
            } else if player_inputs.left.just_pressed {
                if let Some(e) = scale_factor.enum_prev() {
                    *scale_factor = e;
                    scale_changed = true;
                }
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        GameOptionMenuSelection::FPSLimiter => {
            if player_inputs.right.just_pressed {
                if let Some(e) = game_option_menu_data.fps_limiter.enum_next() {
                    game_option_menu_data.fps_limiter = e;
                    fps_changed = true;
                }
            } else if player_inputs.left.just_pressed {
                if let Some(e) = game_option_menu_data.fps_limiter.enum_prev() {
                    game_option_menu_data.fps_limiter = e;
                    fps_changed = true;
                }
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        GameOptionMenuSelection::WindowMode => {
            if player_inputs.right.just_pressed {
                if let Some(e) = game_option_menu_data.window_mode.enum_next() {
                    game_option_menu_data.window_mode = e;
                    window_mode_changed = true;
                }
            } else if player_inputs.left.just_pressed {
                if let Some(e) = game_option_menu_data.window_mode.enum_prev() {
                    game_option_menu_data.window_mode = e;
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
                limiter: game_option_menu_data.fps_limiter.into(),
            };
        }
        option_changed |= fps_changed;
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        if window_mode_changed {
            let mut window = query.single_mut();
            window.mode = game_option_menu_data.window_mode.into();
        }
        option_changed |= window_mode_changed;
    }
    if option_changed {
        play_sound.send(PlaySoundEvent::MoveCursor);
    }
}
