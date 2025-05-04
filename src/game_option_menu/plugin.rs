use bevy::{
    color::palettes::css::{BLUE, WHITE},
    prelude::*,
};
use bevy_dev_tools::fps_overlay::FpsOverlayConfig;
use strum::{EnumCount, IntoEnumIterator};
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
    utility::despawn_all,
};

use super::{game_option::GameOption, scale_factor::ScaleFactor, show_fps::ShowFPS};

#[cfg(all(not(target_arch = "wasm32"), feature = "fps_limiter"))]
use super::fps_limiter::FPSLimiter;

pub fn setup(app: &mut App) {
    #[cfg(all(not(target_arch = "wasm32"), feature = "fps_limiter"))]
    {
        app.add_plugins(bevy_framepace::FramepacePlugin)
            .add_systems(Startup, init_bevy_framepace_settings);
    }
    app.insert_resource(GameOptionMenuData::default())
        .insert_resource(ScaleFactor::default())
        .add_systems(OnEnter(AppState::GameModeMenu), setup_screen)
        .add_systems(
            Update,
            (handle_input_system, update_ui_system)
                .chain()
                .run_if(in_state(AppState::GameModeMenu)),
        )
        .add_systems(
            OnExit(AppState::GameModeMenu),
            despawn_all::<GameOptionMenuEntityMarker>,
        );
}

#[cfg(all(not(target_arch = "wasm32"), feature = "fps_limiter"))]
fn init_bevy_framepace_settings(mut framepace_settins: ResMut<bevy_framepace::FramepaceSettings>) {
    *framepace_settins = bevy_framepace::FramepaceSettings {
        limiter: FPSLimiter::default().into(),
    };
}

#[derive(Component)]
struct GameOptionMenuEntityMarker;

#[derive(Component)]
struct GameOptionEntityMarker(pub GameOptionMenuSelection);

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
    #[cfg(all(not(target_arch = "wasm32"), feature = "fps_limiter"))]
    FPSLimiter,
    ShowFPS,
    ControllerMapping,
    ScaleFactor,
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
    #[cfg(all(not(target_arch = "wasm32"), feature = "fps_limiter"))]
    fps_limiter: FPSLimiter,
    show_fps: ShowFPS,
}

impl GameOptionMenuData {
    pub fn new() -> Self {
        Self {
            selection: GameOptionMenuSelection::default(),
            seed_selection: GameOptionMenuSeedSelection::default(),
            #[cfg(all(not(target_arch = "wasm32"), feature = "fps_limiter"))]
            fps_limiter: FPSLimiter::default(),
            show_fps: ShowFPS::default(),
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
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                overflow: Overflow::clip(),
                ..default()
            },
            GameOptionMenuEntityMarker,
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    display: Display::Grid,
                    grid_template_columns: vec![GridTrack::auto(); TETRIS_BITMAP[0].len()],
                    margin: UiRect::all(Val::Px(30.0)),
                    ..default()
                })
                .with_children(|parent| {
                    TETRIS_BITMAP.iter().for_each(|rows| {
                        rows.iter().for_each(|sqr| {
                            parent.spawn((
                                Node {
                                    width: Val::Px(30.0),
                                    height: Val::Px(30.0),
                                    ..default()
                                },
                                ImageNode::new(logo_images[(*sqr) as usize].clone()),
                            ));
                        })
                    });
                });
            parent
                .spawn((
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::all(Val::Px(30.0)),
                        padding: UiRect::all(Val::Px(30.0)),
                        border: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    BorderColor::from(BLUE),
                ))
                .with_children(|parent| {
                    for selection in GameOptionMenuSelection::iter() {
                        parent.spawn((
                            Text::default(),
                            TextFont::from_font_size(40.0),
                            TextColor::from(WHITE),
                            TextLayout::new(JustifyText::Center, LineBreak::NoWrap),
                            GameOptionEntityMarker(selection),
                        ));
                    }
                });
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
    mut window_query: Query<&mut Window>,
    mut fps_overlay_config: ResMut<FpsOverlayConfig>,
    #[cfg(all(not(target_arch = "wasm32"), feature = "fps_limiter"))] mut framepace_settins: ResMut<
        bevy_framepace::FramepaceSettings,
    >,
) {
    let player_inputs = PlayerInputs::with_keyboard(&keys)
        | PlayerInputs::with_gamepads(gamepads, *controller_mapping);

    if player_inputs.soft_reset {
        play_sound.write(PlaySoundEvent::StartGame);
        app_state.set(AppState::Splash);
        return;
    }

    if player_inputs.b.just_pressed {
        app_state.set(AppState::Splash);
        play_sound.write(PlaySoundEvent::StartGame);
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
                play_sound.write(PlaySoundEvent::MoveCursor);

                return;
            }
            (false, true) => {
                game_option_menu_data.selection = game_option_menu_data.selection.enum_next_cycle();
                play_sound.write(PlaySoundEvent::MoveCursor);
                return;
            }
            _ => (),
        }
    }

    let mut option_changed = false;
    let mut scale_changed = false;

    match game_option_menu_data.selection {
        GameOptionMenuSelection::Tetris => {
            if player_inputs.start.just_pressed {
                play_sound.write(PlaySoundEvent::StartGame);
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
                    GameOptionMenuSeedSelection::Index(index) => {
                        game_config.seed.increment(index);
                        option_changed = true;
                    }
                    GameOptionMenuSeedSelection::None => (),
                }
            } else if player_inputs.down.just_pressed {
                match game_option_menu_data.seed_selection {
                    GameOptionMenuSeedSelection::Index(index) => {
                        game_config.seed.decrement(index);
                        option_changed = true;
                    }
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
        #[cfg(all(not(target_arch = "wasm32"), feature = "fps_limiter"))]
        GameOptionMenuSelection::FPSLimiter => {
            if player_inputs.right.just_pressed {
                if let Some(e) = game_option_menu_data.fps_limiter.enum_next() {
                    game_option_menu_data.fps_limiter = e;
                    framepace_settins.limiter = game_option_menu_data.fps_limiter.into();
                    option_changed = true;
                }
            } else if player_inputs.left.just_pressed {
                if let Some(e) = game_option_menu_data.fps_limiter.enum_prev() {
                    game_option_menu_data.fps_limiter = e;
                    framepace_settins.limiter = game_option_menu_data.fps_limiter.into();
                    option_changed = true;
                }
            }
        }
        GameOptionMenuSelection::ShowFPS => {
            if player_inputs.right.just_pressed {
                if let Some(e) = game_option_menu_data.show_fps.enum_next() {
                    game_option_menu_data.show_fps = e;
                    fps_overlay_config.enabled = game_option_menu_data.show_fps.is_enabled();
                    option_changed = true;
                }
            } else if player_inputs.left.just_pressed {
                if let Some(e) = game_option_menu_data.show_fps.enum_prev() {
                    game_option_menu_data.show_fps = e;
                    fps_overlay_config.enabled = game_option_menu_data.show_fps.is_enabled();
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
    }

    if scale_changed {
        if let Ok(mut window) = window_query.single_mut() {
            window
                .resolution
                .set_scale_factor_override(Some(scale_factor.mul()));
        }
    }
    option_changed |= scale_changed;
    if option_changed {
        play_sound.write(PlaySoundEvent::MoveCursor);
    }
}

fn update_ui_system(
    mut query: Query<(Entity, &GameOptionEntityMarker)>,
    mut tw: TextUiWriter,
    game_option_menu_data: Res<GameOptionMenuData>,
    game_config: Res<GameConfig>,
    controller_mapping: Res<ControllerMapping>,
    scale_factor: Res<ScaleFactor>,
) {
    query.iter_mut().for_each(|(entity, marker)| {
        let fmt = |name: &str, desc: String, left: bool, right: bool| -> String {
            let selected = if marker.0 == game_option_menu_data.selection {
                ">>"
            } else {
                ""
            };
            let l = if left { "<" } else { "" };
            let r = if right { ">" } else { "" };
            format!("{:>2} {:20} {:1} {:18} {:1}", selected, name, l, desc, r)
        };
        match marker.0 {
            GameOptionMenuSelection::Tetris => {
                *tw.text(entity, 0) = fmt("TETRIS", "".into(), false, false);
            }
            GameOptionMenuSelection::Transition => {
                *tw.text(entity, 0) = fmt(
                    "TRANSITION",
                    game_config.transition.desc(),
                    game_config.transition.enum_prev().is_some(),
                    game_config.transition.enum_next().is_some(),
                );
            }
            GameOptionMenuSelection::Linecap => {
                *tw.text(entity, 0) = fmt(
                    "LINECAP",
                    game_config.linecap.desc(),
                    game_config.linecap.enum_prev().is_some(),
                    game_config.linecap.enum_next().is_some(),
                );
            }
            GameOptionMenuSelection::Gravity => {
                *tw.text(entity, 0) = fmt(
                    "GRAVITY",
                    game_config.gravity.desc(),
                    game_config.gravity.enum_prev().is_some(),
                    game_config.gravity.enum_next().is_some(),
                );
            }
            GameOptionMenuSelection::Seeding => {
                *tw.text(entity, 0) = fmt(
                    "SEEDING",
                    game_config.seeding.desc(),
                    game_config.seeding.enum_prev().is_some(),
                    game_config.seeding.enum_next().is_some(),
                );
            }
            GameOptionMenuSelection::Seed => {
                *tw.text(entity, 0) = match game_config.seeding {
                    Seeding::System => fmt("SEED", "".into(), false, false),
                    Seeding::Custom => fmt("SEED", game_config.seed.to_string(), false, false),
                };
            }
            GameOptionMenuSelection::Scoring => {
                *tw.text(entity, 0) = fmt(
                    "SCORING",
                    game_config.scoring.desc(),
                    game_config.scoring.enum_prev().is_some(),
                    game_config.scoring.enum_next().is_some(),
                );
            }
            GameOptionMenuSelection::TVSystem => {
                *tw.text(entity, 0) = fmt(
                    "TV SYSTEM",
                    game_config.tv_system.desc(),
                    game_config.tv_system.enum_prev().is_some(),
                    game_config.tv_system.enum_next().is_some(),
                );
            }
            GameOptionMenuSelection::NextPieceHint => {
                *tw.text(entity, 0) = fmt(
                    "NEXT PIECE HINT",
                    game_config.next_piece_hint.desc(),
                    game_config.next_piece_hint.enum_prev().is_some(),
                    game_config.next_piece_hint.enum_next().is_some(),
                );
            }
            GameOptionMenuSelection::Invisible => {
                *tw.text(entity, 0) = fmt(
                    "INVISIBLE",
                    game_config.invisible.desc(),
                    game_config.invisible.enum_prev().is_some(),
                    game_config.invisible.enum_next().is_some(),
                );
            }
            #[cfg(all(not(target_arch = "wasm32"), feature = "fps_limiter"))]
            GameOptionMenuSelection::FPSLimiter => {
                *tw.text(entity, 0) = fmt(
                    "FPS LIMITER",
                    game_option_menu_data.fps_limiter.desc(),
                    game_option_menu_data.fps_limiter.enum_prev().is_some(),
                    game_option_menu_data.fps_limiter.enum_next().is_some(),
                );
            }
            GameOptionMenuSelection::ShowFPS => {
                *tw.text(entity, 0) = fmt(
                    "SHOW FPS",
                    game_option_menu_data.show_fps.desc(),
                    game_option_menu_data.show_fps.enum_prev().is_some(),
                    game_option_menu_data.show_fps.enum_next().is_some(),
                );
            }
            GameOptionMenuSelection::ControllerMapping => {
                *tw.text(entity, 0) = fmt(
                    "CONTROLLER MAPPING",
                    controller_mapping.desc(),
                    controller_mapping.enum_prev().is_some(),
                    controller_mapping.enum_next().is_some(),
                );
            }
            GameOptionMenuSelection::ScaleFactor => {
                *tw.text(entity, 0) = fmt(
                    "SCALE FACTOR",
                    scale_factor.desc(),
                    scale_factor.enum_prev().is_some(),
                    scale_factor.enum_next().is_some(),
                );
            }
        }
    });
}
