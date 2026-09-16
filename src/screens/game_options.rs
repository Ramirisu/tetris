use bevy::{
    color::palettes::css::{BLUE, WHITE},
    prelude::*,
    window::PrimaryWindow,
};
use bevy_dev_tools::fps_overlay::FpsOverlayConfig;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount, EnumIter, FromRepr};

use crate::{
    app_state::AppState,
    audio::PlaySoundMessage,
    game::{
        game::GameConfig,
        seed::{SEED_HEX_COUNT, Seed},
        seeding::Seeding,
    },
    input::{controller_mapping::ControllerMapping, player_inputs::PlayerInputs},
    logo::logo,
    options::{
        option_name::OptionName,
        scale_factor::{ScaleFactor, WINDOW_HEIGHT, WINDOW_WIDTH},
        show_fps::ShowFPS,
    },
    utility::{effect::flicker, entity::despawn_all, enum_advance, enum_advance_cycle},
};

#[cfg(all(not(target_arch = "wasm32"), feature = "fps_limiter"))]
use crate::options::fps_limiter::FPSLimiter;

#[cfg(not(target_arch = "wasm32"))]
use bevy::{ecs::system::NonSendMarker, winit::WINIT_WINDOWS};

#[cfg(not(target_arch = "wasm32"))]
use crate::options::window_mode::WindowMode;

pub fn plugin(app: &mut App) {
    #[cfg(all(not(target_arch = "wasm32"), feature = "fps_limiter"))]
    {
        app.add_plugins(bevy_framepace::FramepacePlugin)
            .add_systems(Startup, init_bevy_framepace_settings);
    }
    app.insert_resource(OptionsScreenData::default())
        .insert_resource(ScaleFactor::default())
        .add_systems(OnEnter(AppState::GameOptionsScreen), setup_screen)
        .add_systems(
            Update,
            (
                handle_input_system,
                change_window_mode_system,
                update_ui_system,
            )
                .chain()
                .run_if(in_state(AppState::GameOptionsScreen)),
        )
        .add_systems(
            OnExit(AppState::GameOptionsScreen),
            despawn_all::<SettingsMenuEntityMarker>,
        );
}

#[cfg(all(not(target_arch = "wasm32"), feature = "fps_limiter"))]
fn init_bevy_framepace_settings(mut framepace_settins: ResMut<bevy_framepace::FramepaceSettings>) {
    *framepace_settins = bevy_framepace::FramepaceSettings {
        limiter: FPSLimiter::default().into(),
    };
}

#[derive(Component)]
struct SettingsMenuEntityMarker;

#[derive(Component)]
struct SelectedMainSettingEntityMarker(SelectedMainOption, usize);

const FONT_SIZE: f32 = 25.0;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, FromRepr, EnumIter, EnumCount)]
enum SelectedMainOption {
    #[default]
    Tetris,
    Transition,
    Linecap,
    LinecapLevel,
    Gravity,
    Random,
    Seeding,
    Seed,
    ScoreDisplay,
    LevelDisplay,
    TVSystem,
    NextPieceHint,
    Invisible,
    TetrisFlash,
    #[cfg(all(not(target_arch = "wasm32"), feature = "fps_limiter"))]
    FPSLimiter,
    ShowFPS,
    ControllerMapping,
    #[cfg(not(target_arch = "wasm32"))]
    WindowMode,
    ScaleFactor,
    #[cfg(not(target_arch = "wasm32"))]
    Exit,
}

impl SelectedMainOption {
    pub fn name(&self) -> std::borrow::Cow<'_, str> {
        match *self {
            SelectedMainOption::Tetris => "TETRIS".into(),
            SelectedMainOption::Transition => t!("tetris.options.transition"),
            SelectedMainOption::Linecap => t!("tetris.options.linecap"),
            SelectedMainOption::LinecapLevel => t!("tetris.options.linecap_level"),
            SelectedMainOption::Gravity => t!("tetris.options.gravity"),
            SelectedMainOption::Random => t!("tetris.options.random"),
            SelectedMainOption::Seeding => t!("tetris.options.seeding"),
            SelectedMainOption::Seed => t!("tetris.options.seed"),
            SelectedMainOption::ScoreDisplay => t!("tetris.options.score_display"),
            SelectedMainOption::LevelDisplay => t!("tetris.options.level_display"),
            SelectedMainOption::TVSystem => t!("tetris.options.tv_system"),
            SelectedMainOption::NextPieceHint => t!("tetris.options.next_piece_hint"),
            SelectedMainOption::Invisible => t!("tetris.options.invisible"),
            SelectedMainOption::TetrisFlash => t!("tetris.options.tetris_flash"),
            #[cfg(all(not(target_arch = "wasm32"), feature = "fps_limiter"))]
            SelectedMainOption::FPSLimiter => t!("tetris.options.fps_limiter"),
            SelectedMainOption::ShowFPS => t!("tetris.options.show_fps"),
            SelectedMainOption::ControllerMapping => {
                t!("tetris.options.controller_mapping")
            }
            #[cfg(not(target_arch = "wasm32"))]
            SelectedMainOption::WindowMode => t!("tetris.options.window_mode"),
            SelectedMainOption::ScaleFactor => t!("tetris.options.scale_factor"),
            #[cfg(not(target_arch = "wasm32"))]
            SelectedMainOption::Exit => t!("tetris.options.exit"),
        }
    }
}

enum_advance::enum_advance_derive!(SelectedMainOption);
enum_advance_cycle::enum_advance_cycle_derive!(SelectedMainOption);

const SEED_FIRST: usize = 0;
const SEED_LAST: usize = SEED_HEX_COUNT - 1;

#[derive(Resource)]
struct OptionsScreenData {
    selected_main_option: SelectedMainOption,
    selected_seed_option: Option<usize>,
    #[cfg(all(not(target_arch = "wasm32"), feature = "fps_limiter"))]
    fps_limiter: FPSLimiter,
    show_fps: ShowFPS,
    #[cfg(not(target_arch = "wasm32"))]
    window_mode: WindowMode,
    scale_changed: bool,
}

impl OptionsScreenData {
    pub fn new() -> Self {
        Self {
            selected_main_option: SelectedMainOption::default(),
            selected_seed_option: None,
            #[cfg(all(not(target_arch = "wasm32"), feature = "fps_limiter"))]
            fps_limiter: FPSLimiter::default(),
            show_fps: ShowFPS::default(),
            #[cfg(not(target_arch = "wasm32"))]
            window_mode: WindowMode::default(),
            scale_changed: false,
        }
    }
}

impl Default for OptionsScreenData {
    fn default() -> Self {
        Self::new()
    }
}

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
                overflow: Overflow::clip(),
                ..default()
            },
            SettingsMenuEntityMarker,
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
                        display: Display::Grid,
                        grid_template_columns: vec![GridTrack::auto(); 5],
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(20.0),
                        row_gap: Val::Px(5.0),
                        margin: UiRect::all(Val::Px(20.0)),
                        padding: UiRect::all(Val::Px(20.0)),
                        border: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    BorderColor::from(BLUE),
                ))
                .with_children(|p| {
                    for selected_main_option in SelectedMainOption::iter() {
                        let cols: [(String, Val, f32); 5] = [
                            ("▶".into(), Val::Auto, 15.0),
                            (
                                selected_main_option.name().into(),
                                Val::Px(300.0),
                                FONT_SIZE,
                            ),
                            ("".into(), Val::Auto, FONT_SIZE),
                            ("".into(), Val::Px(300.0), FONT_SIZE),
                            ("".into(), Val::Auto, FONT_SIZE),
                        ];

                        for (idx, (name, width, font_size)) in cols.iter().enumerate() {
                            let mut ec = p.spawn((
                                Node {
                                    width: *width,
                                    height: Val::Auto,
                                    ..default()
                                },
                                Text::new(name),
                                TextFont::from_font_size(*font_size),
                                TextColor::from(WHITE),
                                TextLayout::new(Justify::Center, LineBreak::NoWrap),
                                SelectedMainSettingEntityMarker(selected_main_option, idx),
                            ));

                            if selected_main_option == SelectedMainOption::Seed && idx == 3 {
                                ec.with_children(|p| {
                                    for _ in 0..SEED_HEX_COUNT {
                                        p.spawn((
                                            TextSpan::default(),
                                            TextFont::from_font_size(*font_size),
                                            TextColor::from(WHITE),
                                            TextLayout::new(Justify::Center, LineBreak::NoWrap),
                                        ));
                                    }
                                });
                            }
                        }
                    }
                });
            });
        });
}

fn handle_input_system(
    keys: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    mut controller_mapping: ResMut<ControllerMapping>,
    mut options_screen_data: ResMut<OptionsScreenData>,
    mut game_config: ResMut<GameConfig>,
    mut app_state: ResMut<NextState<AppState>>,
    mut play_sound: MessageWriter<PlaySoundMessage>,
    mut scale_factor: ResMut<ScaleFactor>,
    mut fps_overlay_config: ResMut<FpsOverlayConfig>,
    #[cfg(all(not(target_arch = "wasm32"), feature = "fps_limiter"))] mut framepace_settins: ResMut<
        bevy_framepace::FramepaceSettings,
    >,
    #[cfg(not(target_arch = "wasm32"))] mut exit: MessageWriter<AppExit>,
) {
    let player_inputs = PlayerInputs::with_keyboard(&keys)
        | PlayerInputs::with_gamepads(gamepads, *controller_mapping);

    if player_inputs.soft_reset {
        play_sound.write(PlaySoundMessage::StartGame);
        app_state.set(AppState::SplashScreen);
        return;
    }

    if player_inputs.b.just_pressed {
        play_sound.write(PlaySoundMessage::StartGame);
        app_state.set(AppState::LanguageScreen);
        return;
    }

    if options_screen_data.selected_main_option != SelectedMainOption::Seed
        || game_config.seeding == Seeding::System
        || options_screen_data.selected_seed_option.is_none()
    {
        match (
            player_inputs.up.just_pressed,
            player_inputs.down.just_pressed,
        ) {
            (true, false) => {
                options_screen_data.selected_main_option =
                    options_screen_data.selected_main_option.enum_prev_cycle();
                play_sound.write(PlaySoundMessage::MoveCursor);

                return;
            }
            (false, true) => {
                options_screen_data.selected_main_option =
                    options_screen_data.selected_main_option.enum_next_cycle();
                play_sound.write(PlaySoundMessage::MoveCursor);
                return;
            }
            _ => (),
        }
    }

    let mut option_changed = false;

    match options_screen_data.selected_main_option {
        SelectedMainOption::Tetris => {
            if player_inputs.start.just_pressed {
                play_sound.write(PlaySoundMessage::StartGame);
                app_state.set(AppState::GameLevelsScreen);
            }
        }
        SelectedMainOption::Transition => {
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
        SelectedMainOption::Linecap => {
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
        SelectedMainOption::LinecapLevel => {
            if game_config.linecap != crate::game::linecap::Linecap::Off {
                if player_inputs.right.just_pressed {
                    game_config.linecap_level += 1;
                    option_changed = true;
                } else if player_inputs.left.just_pressed {
                    if game_config.linecap_level > 0 {
                        game_config.linecap_level -= 1;
                        option_changed = true;
                    }
                }
            }
        }
        SelectedMainOption::Gravity => {
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
        SelectedMainOption::Random => {
            if player_inputs.right.just_pressed {
                if let Some(e) = game_config.random.enum_next() {
                    game_config.random = e;
                    option_changed = true;
                }
            } else if player_inputs.left.just_pressed {
                if let Some(e) = game_config.random.enum_prev() {
                    game_config.random = e;
                    option_changed = true;
                }
            }
        }
        SelectedMainOption::Seeding => {
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
        SelectedMainOption::Seed => {
            if game_config.seeding == Seeding::Custom && player_inputs.start.just_pressed {
                match options_screen_data.selected_seed_option {
                    None => {
                        options_screen_data.selected_seed_option = Some(SEED_LAST);
                    }
                    Some(_) => {
                        options_screen_data.selected_seed_option = None;
                    }
                }
                option_changed = true;
            } else if player_inputs.right.just_pressed {
                match options_screen_data.selected_seed_option {
                    Some(SEED_FIRST) => (),
                    Some(index) => {
                        options_screen_data.selected_seed_option = Some(index - 1);
                        option_changed = true;
                    }
                    None => (),
                }
            } else if player_inputs.left.just_pressed {
                match options_screen_data.selected_seed_option {
                    Some(SEED_LAST) => (),
                    Some(index) => {
                        options_screen_data.selected_seed_option = Some(index + 1);
                        option_changed = true;
                    }
                    None => (),
                }
            } else if player_inputs.up.just_pressed {
                match options_screen_data.selected_seed_option {
                    Some(index) => {
                        game_config.seed.increment(index);
                        option_changed = true;
                    }
                    None => (),
                }
            } else if player_inputs.down.just_pressed {
                match options_screen_data.selected_seed_option {
                    Some(index) => {
                        game_config.seed.decrement(index);
                        option_changed = true;
                    }
                    None => (),
                }
            } else if player_inputs.select.just_pressed {
                game_config.seed = Seed::new();
            }
        }
        SelectedMainOption::ScoreDisplay => {
            if player_inputs.right.just_pressed {
                if let Some(e) = game_config.score_display.enum_next() {
                    game_config.score_display = e;
                    option_changed = true;
                }
            } else if player_inputs.left.just_pressed {
                if let Some(e) = game_config.score_display.enum_prev() {
                    game_config.score_display = e;
                    option_changed = true;
                }
            }
        }
        SelectedMainOption::LevelDisplay => {
            if player_inputs.right.just_pressed {
                if let Some(e) = game_config.level_display.enum_next() {
                    game_config.level_display = e;
                    option_changed = true;
                }
            } else if player_inputs.left.just_pressed {
                if let Some(e) = game_config.level_display.enum_prev() {
                    game_config.level_display = e;
                    option_changed = true;
                }
            }
        }
        SelectedMainOption::TVSystem => {
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
        SelectedMainOption::NextPieceHint => {
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
        SelectedMainOption::Invisible => {
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
        SelectedMainOption::TetrisFlash => {
            if player_inputs.right.just_pressed {
                if let Some(e) = game_config.tetris_flash.enum_next() {
                    game_config.tetris_flash = e;
                    option_changed = true;
                }
            } else if player_inputs.left.just_pressed {
                if let Some(e) = game_config.tetris_flash.enum_prev() {
                    game_config.tetris_flash = e;
                    option_changed = true;
                }
            }
        }
        #[cfg(all(not(target_arch = "wasm32"), feature = "fps_limiter"))]
        SelectedMainOption::FPSLimiter => {
            if player_inputs.right.just_pressed {
                if let Some(e) = options_screen_data.fps_limiter.enum_next() {
                    options_screen_data.fps_limiter = e;
                    framepace_settins.limiter = options_screen_data.fps_limiter.into();
                    option_changed = true;
                }
            } else if player_inputs.left.just_pressed {
                if let Some(e) = options_screen_data.fps_limiter.enum_prev() {
                    options_screen_data.fps_limiter = e;
                    framepace_settins.limiter = options_screen_data.fps_limiter.into();
                    option_changed = true;
                }
            }
        }
        SelectedMainOption::ShowFPS => {
            if player_inputs.right.just_pressed {
                if let Some(e) = options_screen_data.show_fps.enum_next() {
                    options_screen_data.show_fps = e;
                    fps_overlay_config.enabled = options_screen_data.show_fps.is_enabled();
                    option_changed = true;
                }
            } else if player_inputs.left.just_pressed {
                if let Some(e) = options_screen_data.show_fps.enum_prev() {
                    options_screen_data.show_fps = e;
                    fps_overlay_config.enabled = options_screen_data.show_fps.is_enabled();
                    option_changed = true;
                }
            }
        }
        SelectedMainOption::ControllerMapping => {
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
        #[cfg(not(target_arch = "wasm32"))]
        SelectedMainOption::WindowMode => {
            if player_inputs.right.just_pressed {
                if let Some(e) = options_screen_data.window_mode.enum_next() {
                    options_screen_data.window_mode = e;
                    options_screen_data.scale_changed = true;
                }
            } else if player_inputs.left.just_pressed {
                if let Some(e) = options_screen_data.window_mode.enum_prev() {
                    options_screen_data.window_mode = e;
                    options_screen_data.scale_changed = true;
                }
            }
        }
        SelectedMainOption::ScaleFactor => {
            if player_inputs.right.just_pressed {
                if let Some(e) = scale_factor.enum_next() {
                    *scale_factor = e;
                    options_screen_data.scale_changed = true;
                }
            } else if player_inputs.left.just_pressed {
                if let Some(e) = scale_factor.enum_prev() {
                    *scale_factor = e;
                    options_screen_data.scale_changed = true;
                }
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        SelectedMainOption::Exit => {
            if player_inputs.start.just_pressed {
                exit.write(AppExit::Success);
            }
        }
    }

    option_changed |= options_screen_data.scale_changed;
    if option_changed {
        play_sound.write(PlaySoundMessage::MoveCursor);
    }
}

fn change_window_mode_system(
    mut options_screen_data: ResMut<OptionsScreenData>,
    scale_factor: Res<ScaleFactor>,
    mut q: ParamSet<(Query<Entity, With<PrimaryWindow>>, Query<&mut Window>)>,
    mut ui_scale: ResMut<UiScale>,
    #[cfg(not(target_arch = "wasm32"))] _marker: NonSendMarker,
) {
    if !std::mem::replace(&mut options_screen_data.scale_changed, false) {
        return;
    }

    #[cfg(not(target_arch = "wasm32"))]
    let monitor = q.p0().single().ok().and_then(|entity| {
        WINIT_WINDOWS.with_borrow(|winit_windows| {
            winit_windows
                .get_window(entity)
                .and_then(|winit_window| winit_window.current_monitor())
        })
    });

    if let Ok(mut window) = q.p1().single_mut() {
        #[cfg(target_arch = "wasm32")]
        {
            window.resolution.set_physical_resolution(
                (WINDOW_WIDTH * scale_factor.mul()) as u32,
                (WINDOW_HEIGHT * scale_factor.mul()) as u32,
            );
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            if options_screen_data.window_mode != WindowMode::Windowed && monitor.is_some() {
                // borderless fullscreen requires the current monitor's physical size to be known.
                let monitor = monitor.unwrap();
                info!("Current monitor: {:?}", monitor.size());
                window
                    .resolution
                    .set_physical_resolution(monitor.size().width, monitor.size().height);
            } else {
                // fallback: set it to windowed mode.
                options_screen_data.window_mode = WindowMode::Windowed;
                window.resolution.set_physical_resolution(
                    (WINDOW_WIDTH * scale_factor.mul()) as u32,
                    (WINDOW_HEIGHT * scale_factor.mul()) as u32,
                );
            }

            window.mode = options_screen_data.window_mode.into();
        }
    }

    ui_scale.0 = scale_factor.mul();
}

fn update_ui_system(
    t: Res<Time>,
    q: Query<(Entity, &SelectedMainSettingEntityMarker)>,
    mut tw: TextUiWriter,
    options_screen_data: Res<OptionsScreenData>,
    game_config: Res<GameConfig>,
    controller_mapping: Res<ControllerMapping>,
    scale_factor: Res<ScaleFactor>,
) {
    for (entity, marker) in q {
        let fmt_selected = |tw: &mut TextUiWriter| {
            tw.color(entity, 0).set_alpha(
                if marker.0 == options_screen_data.selected_main_option {
                    flicker(t.elapsed_secs(), 0.5)
                } else {
                    0.0
                },
            );
        };
        let fmt_larrow = |tw: &mut TextUiWriter, b: bool| {
            *tw.text(entity, 0) = (if b { "<" } else { " " }).into()
        };
        let fmt_rarrow = |tw: &mut TextUiWriter, b: bool| {
            *tw.text(entity, 0) = (if b { ">" } else { " " }).into()
        };
        let fmt_desc = |tw: &mut TextUiWriter, desc: String| *tw.text(entity, 0) = desc;
        match (marker.0, marker.1) {
            (SelectedMainOption::Tetris, 2) => (),
            (SelectedMainOption::Tetris, 3) => (),
            (SelectedMainOption::Tetris, 4) => (),
            (SelectedMainOption::Transition, 2) => {
                fmt_larrow(&mut tw, game_config.transition.enum_prev().is_some())
            }
            (SelectedMainOption::Transition, 3) => fmt_desc(&mut tw, game_config.transition.name()),
            (SelectedMainOption::Transition, 4) => {
                fmt_rarrow(&mut tw, game_config.transition.enum_next().is_some())
            }
            (SelectedMainOption::Linecap, 2) => {
                fmt_larrow(&mut tw, game_config.linecap.enum_prev().is_some())
            }
            (SelectedMainOption::Linecap, 3) => fmt_desc(&mut tw, game_config.linecap.name()),
            (SelectedMainOption::Linecap, 4) => {
                fmt_rarrow(&mut tw, game_config.linecap.enum_next().is_some())
            }
            (SelectedMainOption::LinecapLevel, 2) => fmt_larrow(
                &mut tw,
                game_config.linecap != crate::game::linecap::Linecap::Off
                    && game_config.linecap_level > 0,
            ),
            (SelectedMainOption::LinecapLevel, 3) => match game_config.linecap {
                crate::game::linecap::Linecap::Off => fmt_desc(&mut tw, "".into()),
                crate::game::linecap::Linecap::KillScreenX2
                | crate::game::linecap::Linecap::Halt => {
                    fmt_desc(&mut tw, format!("{:02}", game_config.linecap_level.0))
                }
            },
            (SelectedMainOption::LinecapLevel, 4) => fmt_rarrow(
                &mut tw,
                game_config.linecap != crate::game::linecap::Linecap::Off,
            ),
            (SelectedMainOption::Gravity, 2) => {
                fmt_larrow(&mut tw, game_config.gravity.enum_prev().is_some())
            }
            (SelectedMainOption::Gravity, 3) => fmt_desc(&mut tw, game_config.gravity.name()),
            (SelectedMainOption::Gravity, 4) => {
                fmt_rarrow(&mut tw, game_config.gravity.enum_next().is_some())
            }
            (SelectedMainOption::Random, 2) => {
                fmt_larrow(&mut tw, game_config.random.enum_prev().is_some())
            }
            (SelectedMainOption::Random, 3) => fmt_desc(&mut tw, game_config.random.name()),
            (SelectedMainOption::Random, 4) => {
                fmt_rarrow(&mut tw, game_config.random.enum_next().is_some())
            }
            (SelectedMainOption::Seeding, 2) => {
                fmt_larrow(&mut tw, game_config.seeding.enum_prev().is_some())
            }
            (SelectedMainOption::Seeding, 3) => fmt_desc(&mut tw, game_config.seeding.name()),
            (SelectedMainOption::Seeding, 4) => {
                fmt_rarrow(&mut tw, game_config.seeding.enum_next().is_some())
            }
            (SelectedMainOption::Seed, 2) => fmt_larrow(&mut tw, false),
            (SelectedMainOption::Seed, 3) => match game_config.seeding {
                Seeding::System => {
                    for idx in 0..=SEED_HEX_COUNT {
                        *tw.text(entity, idx) = "".into();
                        tw.font(entity, idx).font_size = FontSize::Px(FONT_SIZE);
                    }
                }
                Seeding::Custom => {
                    for (byte_idx, byte) in game_config.seed.bytes.iter().enumerate() {
                        for (hex_idx, hex) in [byte & 0xf, byte >> 4].iter().enumerate() {
                            let idx = byte_idx * 2 + hex_idx;
                            *tw.text(entity, SEED_HEX_COUNT - idx) = format!("{:X}", *hex);
                            tw.font(entity, SEED_HEX_COUNT - idx).font_size = if options_screen_data
                                .selected_seed_option
                                .map_or(false, |selected| selected == idx)
                            {
                                FontSize::Px(FONT_SIZE * 2.0)
                            } else {
                                FontSize::Px(FONT_SIZE)
                            };
                        }
                    }
                }
            },
            (SelectedMainOption::Seed, 4) => fmt_rarrow(&mut tw, false),
            (SelectedMainOption::ScoreDisplay, 2) => {
                fmt_larrow(&mut tw, game_config.score_display.enum_prev().is_some())
            }
            (SelectedMainOption::ScoreDisplay, 3) => {
                fmt_desc(&mut tw, game_config.score_display.name())
            }
            (SelectedMainOption::ScoreDisplay, 4) => {
                fmt_rarrow(&mut tw, game_config.score_display.enum_next().is_some())
            }
            (SelectedMainOption::LevelDisplay, 2) => {
                fmt_larrow(&mut tw, game_config.level_display.enum_prev().is_some())
            }
            (SelectedMainOption::LevelDisplay, 3) => {
                fmt_desc(&mut tw, game_config.level_display.name())
            }
            (SelectedMainOption::LevelDisplay, 4) => {
                fmt_rarrow(&mut tw, game_config.level_display.enum_next().is_some())
            }
            (SelectedMainOption::TVSystem, 2) => {
                fmt_larrow(&mut tw, game_config.tv_system.enum_prev().is_some())
            }
            (SelectedMainOption::TVSystem, 3) => fmt_desc(&mut tw, game_config.tv_system.name()),
            (SelectedMainOption::TVSystem, 4) => {
                fmt_rarrow(&mut tw, game_config.tv_system.enum_next().is_some())
            }
            (SelectedMainOption::NextPieceHint, 2) => {
                fmt_larrow(&mut tw, game_config.next_piece_hint.enum_prev().is_some())
            }
            (SelectedMainOption::NextPieceHint, 3) => {
                fmt_desc(&mut tw, game_config.next_piece_hint.name())
            }
            (SelectedMainOption::NextPieceHint, 4) => {
                fmt_rarrow(&mut tw, game_config.next_piece_hint.enum_next().is_some())
            }
            (SelectedMainOption::Invisible, 2) => {
                fmt_larrow(&mut tw, game_config.invisible.enum_prev().is_some())
            }
            (SelectedMainOption::Invisible, 3) => fmt_desc(&mut tw, game_config.invisible.name()),
            (SelectedMainOption::Invisible, 4) => {
                fmt_rarrow(&mut tw, game_config.invisible.enum_next().is_some())
            }
            (SelectedMainOption::TetrisFlash, 2) => {
                fmt_larrow(&mut tw, game_config.tetris_flash.enum_prev().is_some())
            }
            (SelectedMainOption::TetrisFlash, 3) => {
                fmt_desc(&mut tw, game_config.tetris_flash.name())
            }
            (SelectedMainOption::TetrisFlash, 4) => {
                fmt_rarrow(&mut tw, game_config.tetris_flash.enum_next().is_some())
            }
            #[cfg(all(not(target_arch = "wasm32"), feature = "fps_limiter"))]
            (SelectedMainOption::FPSLimiter, 2) => fmt_larrow(
                &mut tw,
                options_screen_data.fps_limiter.enum_prev().is_some(),
            ),
            #[cfg(all(not(target_arch = "wasm32"), feature = "fps_limiter"))]
            (SelectedMainOption::FPSLimiter, 3) => {
                fmt_desc(&mut tw, options_screen_data.fps_limiter.name())
            }
            #[cfg(all(not(target_arch = "wasm32"), feature = "fps_limiter"))]
            (SelectedMainOption::FPSLimiter, 4) => fmt_rarrow(
                &mut tw,
                options_screen_data.fps_limiter.enum_next().is_some(),
            ),
            (SelectedMainOption::ShowFPS, 2) => {
                fmt_larrow(&mut tw, options_screen_data.show_fps.enum_prev().is_some())
            }
            (SelectedMainOption::ShowFPS, 3) => {
                fmt_desc(&mut tw, options_screen_data.show_fps.name())
            }
            (SelectedMainOption::ShowFPS, 4) => {
                fmt_rarrow(&mut tw, options_screen_data.show_fps.enum_next().is_some())
            }
            (SelectedMainOption::ControllerMapping, 2) => {
                fmt_larrow(&mut tw, controller_mapping.enum_prev().is_some())
            }
            (SelectedMainOption::ControllerMapping, 3) => {
                fmt_desc(&mut tw, controller_mapping.name())
            }
            (SelectedMainOption::ControllerMapping, 4) => {
                fmt_rarrow(&mut tw, controller_mapping.enum_next().is_some())
            }
            #[cfg(not(target_arch = "wasm32"))]
            (SelectedMainOption::WindowMode, 2) => fmt_larrow(
                &mut tw,
                options_screen_data.window_mode.enum_prev().is_some(),
            ),
            #[cfg(not(target_arch = "wasm32"))]
            (SelectedMainOption::WindowMode, 3) => {
                fmt_desc(&mut tw, options_screen_data.window_mode.name())
            }
            #[cfg(not(target_arch = "wasm32"))]
            (SelectedMainOption::WindowMode, 4) => fmt_rarrow(
                &mut tw,
                options_screen_data.window_mode.enum_next().is_some(),
            ),
            (SelectedMainOption::ScaleFactor, 2) => {
                fmt_larrow(&mut tw, scale_factor.enum_prev().is_some())
            }
            (SelectedMainOption::ScaleFactor, 3) => fmt_desc(&mut tw, scale_factor.name()),
            (SelectedMainOption::ScaleFactor, 4) => {
                fmt_rarrow(&mut tw, scale_factor.enum_next().is_some())
            }
            #[cfg(not(target_arch = "wasm32"))]
            (SelectedMainOption::Exit, 2) => (),
            #[cfg(not(target_arch = "wasm32"))]
            (SelectedMainOption::Exit, 3) => (),
            #[cfg(not(target_arch = "wasm32"))]
            (SelectedMainOption::Exit, 4) => (),
            (_, 0) => fmt_selected(&mut tw),
            (_, 1) => (),
            (select, idx) => unreachable!("unimplemented option: ({:?}, {})", select, idx),
        }
    }
}
