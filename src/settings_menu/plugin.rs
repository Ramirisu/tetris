use bevy::{
    color::palettes::css::{BLUE, WHITE},
    ecs::spawn::SpawnWith,
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
    logo::logo,
    utility::entity::despawn_all,
};

use super::{scale_factor::ScaleFactor, setting_name::SettingName, show_fps::ShowFPS};

#[cfg(all(not(target_arch = "wasm32"), feature = "fps_limiter"))]
use super::fps_limiter::FPSLimiter;

pub fn setup(app: &mut App) {
    #[cfg(all(not(target_arch = "wasm32"), feature = "fps_limiter"))]
    {
        app.add_plugins(bevy_framepace::FramepacePlugin)
            .add_systems(Startup, init_bevy_framepace_settings);
    }
    app.insert_resource(SettingsMenuData::default())
        .insert_resource(ScaleFactor::default())
        .add_systems(OnEnter(AppState::SettingsMenu), setup_screen)
        .add_systems(
            Update,
            (handle_input_system, update_ui_system)
                .chain()
                .run_if(in_state(AppState::SettingsMenu)),
        )
        .add_systems(
            OnExit(AppState::SettingsMenu),
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
struct SelectedMainSettingEntityMarker(SelectedMainSetting, usize);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, FromRepr, EnumIter, EnumCount)]
enum SelectedMainSetting {
    #[default]
    Tetris,
    Transition,
    Linecap,
    LinecapLevel,
    Gravity,
    Seeding,
    Seed,
    Scoring,
    Leveling,
    TVSystem,
    NextPieceHint,
    Invisible,
    #[cfg(all(not(target_arch = "wasm32"), feature = "fps_limiter"))]
    FPSLimiter,
    ShowFPS,
    ControllerMapping,
    ScaleFactor,
}

impl SelectedMainSetting {
    pub fn name(&self) -> std::borrow::Cow<'_, str> {
        match *self {
            SelectedMainSetting::Tetris => "TETRIS".into(),
            SelectedMainSetting::Transition => t!("tetris.settings.transition"),
            SelectedMainSetting::Linecap => t!("tetris.settings.linecap"),
            SelectedMainSetting::LinecapLevel => t!("tetris.settings.linecap_level"),
            SelectedMainSetting::Gravity => t!("tetris.settings.gravity"),
            SelectedMainSetting::Seeding => t!("tetris.settings.seeding"),
            SelectedMainSetting::Seed => t!("tetris.settings.seed"),
            SelectedMainSetting::Scoring => t!("tetris.settings.scoring"),
            SelectedMainSetting::Leveling => t!("tetris.settings.leveling"),
            SelectedMainSetting::TVSystem => t!("tetris.settings.tv_system"),
            SelectedMainSetting::NextPieceHint => t!("tetris.settings.next_piece_hint"),
            SelectedMainSetting::Invisible => t!("tetris.settings.invisible"),
            #[cfg(all(not(target_arch = "wasm32"), feature = "fps_limiter"))]
            SelectedMainSetting::FPSLimiter => t!("tetris.settings.fps_limiter"),
            SelectedMainSetting::ShowFPS => t!("tetris.settings.show_fps"),
            SelectedMainSetting::ControllerMapping => {
                t!("tetris.settings.controller_mapping")
            }
            SelectedMainSetting::ScaleFactor => t!("tetris.settings.scale_factor"),
        }
    }
}

enum_advance::enum_advance_derive!(SelectedMainSetting);
enum_advance_cycle::enum_advance_cycle_derive!(SelectedMainSetting);

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum SelectedSeedSetting {
    #[default]
    None,
    Index(usize),
}

const SEED_LEN: usize = SEED_BYTES_USED * 2;
const SEED_FIRST: usize = 0;
const SEED_LAST: usize = SEED_LEN - 1;

#[derive(Resource)]
struct SettingsMenuData {
    selected_main_setting: SelectedMainSetting,
    selected_seed_setting: SelectedSeedSetting,
    #[cfg(all(not(target_arch = "wasm32"), feature = "fps_limiter"))]
    fps_limiter: FPSLimiter,
    show_fps: ShowFPS,
}

impl SettingsMenuData {
    pub fn new() -> Self {
        Self {
            selected_main_setting: SelectedMainSetting::default(),
            selected_seed_setting: SelectedSeedSetting::default(),
            #[cfg(all(not(target_arch = "wasm32"), feature = "fps_limiter"))]
            fps_limiter: FPSLimiter::default(),
            show_fps: ShowFPS::default(),
        }
    }
}

impl Default for SettingsMenuData {
    fn default() -> Self {
        Self::new()
    }
}

fn setup_screen(mut commands: Commands, mut image_assets: ResMut<Assets<Image>>) {
    commands.spawn((
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
        Children::spawn((
            Spawn((
                Node {
                    margin: UiRect::all(Val::Px(40.0)),
                    ..default()
                },
                Children::spawn(Spawn(logo(Val::Px(20.0), &mut image_assets))),
            )),
            Spawn((
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
                Children::spawn(SpawnWith(|p: &mut ChildSpawner| {
                    for selected_main_setting in SelectedMainSetting::iter() {
                        let cols: [(String, Val); 5] = [
                            ("".into(), Val::Auto),
                            (selected_main_setting.name().into(), Val::Px(300.0)),
                            ("".into(), Val::Auto),
                            ("".into(), Val::Px(300.0)),
                            ("".into(), Val::Auto),
                        ];

                        for (idx, (name, width)) in cols.iter().enumerate() {
                            p.spawn((
                                Node {
                                    width: *width,
                                    height: Val::Auto,
                                    ..default()
                                },
                                Text::new(name),
                                TextFont::from_font_size(30.0),
                                TextColor::from(WHITE),
                                TextLayout::new(JustifyText::Center, LineBreak::NoWrap),
                                SelectedMainSettingEntityMarker(selected_main_setting, idx),
                            ));
                        }
                    }
                })),
            )),
        )),
    ));
}

fn handle_input_system(
    keys: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    mut controller_mapping: ResMut<ControllerMapping>,
    mut settings_menu_data: ResMut<SettingsMenuData>,
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
        app_state.set(AppState::SplashScreen);
        return;
    }

    if player_inputs.b.just_pressed {
        play_sound.write(PlaySoundEvent::StartGame);
        app_state.set(AppState::LanguageMenu);
        return;
    }

    if settings_menu_data.selected_main_setting != SelectedMainSetting::Seed
        || game_config.seeding == Seeding::System
        || settings_menu_data.selected_seed_setting == SelectedSeedSetting::None
    {
        match (
            player_inputs.up.just_pressed,
            player_inputs.down.just_pressed,
        ) {
            (true, false) => {
                settings_menu_data.selected_main_setting =
                    settings_menu_data.selected_main_setting.enum_prev_cycle();
                play_sound.write(PlaySoundEvent::MoveCursor);

                return;
            }
            (false, true) => {
                settings_menu_data.selected_main_setting =
                    settings_menu_data.selected_main_setting.enum_next_cycle();
                play_sound.write(PlaySoundEvent::MoveCursor);
                return;
            }
            _ => (),
        }
    }

    let mut option_changed = false;
    let mut scale_changed = false;

    match settings_menu_data.selected_main_setting {
        SelectedMainSetting::Tetris => {
            if player_inputs.start.just_pressed {
                play_sound.write(PlaySoundEvent::StartGame);
                app_state.set(AppState::LevelMenu);
            }
        }
        SelectedMainSetting::Transition => {
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
        SelectedMainSetting::Linecap => {
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
        SelectedMainSetting::LinecapLevel => {
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
        SelectedMainSetting::Gravity => {
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
        SelectedMainSetting::Seeding => {
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
        SelectedMainSetting::Seed => {
            if player_inputs.start.just_pressed {
                match settings_menu_data.selected_seed_setting {
                    SelectedSeedSetting::None => {
                        settings_menu_data.selected_seed_setting =
                            SelectedSeedSetting::Index(SEED_LAST);
                    }
                    SelectedSeedSetting::Index(_) => {
                        settings_menu_data.selected_seed_setting = SelectedSeedSetting::None;
                    }
                }
                option_changed = true;
            } else if player_inputs.right.just_pressed {
                match settings_menu_data.selected_seed_setting {
                    SelectedSeedSetting::Index(SEED_FIRST) => (),
                    SelectedSeedSetting::Index(index) => {
                        settings_menu_data.selected_seed_setting =
                            SelectedSeedSetting::Index(index - 1);
                        option_changed = true;
                    }
                    SelectedSeedSetting::None => (),
                }
            } else if player_inputs.left.just_pressed {
                match settings_menu_data.selected_seed_setting {
                    SelectedSeedSetting::Index(SEED_LAST) => (),
                    SelectedSeedSetting::Index(index) => {
                        settings_menu_data.selected_seed_setting =
                            SelectedSeedSetting::Index(index + 1);
                        option_changed = true;
                    }
                    SelectedSeedSetting::None => (),
                }
            } else if player_inputs.up.just_pressed {
                match settings_menu_data.selected_seed_setting {
                    SelectedSeedSetting::Index(index) => {
                        game_config.seed.increment(index);
                        option_changed = true;
                    }
                    SelectedSeedSetting::None => (),
                }
            } else if player_inputs.down.just_pressed {
                match settings_menu_data.selected_seed_setting {
                    SelectedSeedSetting::Index(index) => {
                        game_config.seed.decrement(index);
                        option_changed = true;
                    }
                    SelectedSeedSetting::None => (),
                }
            } else if player_inputs.select.just_pressed {
                game_config.seed = Seed::new();
            }
        }
        SelectedMainSetting::Scoring => {
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

        SelectedMainSetting::Leveling => {
            if player_inputs.right.just_pressed {
                if let Some(e) = game_config.leveling.enum_next() {
                    game_config.leveling = e;
                    option_changed = true;
                }
            } else if player_inputs.left.just_pressed {
                if let Some(e) = game_config.leveling.enum_prev() {
                    game_config.leveling = e;
                    option_changed = true;
                }
            }
        }
        SelectedMainSetting::TVSystem => {
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
        SelectedMainSetting::NextPieceHint => {
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
        SelectedMainSetting::Invisible => {
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
        SelectedMainSetting::FPSLimiter => {
            if player_inputs.right.just_pressed {
                if let Some(e) = settings_menu_data.fps_limiter.enum_next() {
                    settings_menu_data.fps_limiter = e;
                    framepace_settins.limiter = settings_menu_data.fps_limiter.into();
                    option_changed = true;
                }
            } else if player_inputs.left.just_pressed {
                if let Some(e) = settings_menu_data.fps_limiter.enum_prev() {
                    settings_menu_data.fps_limiter = e;
                    framepace_settins.limiter = settings_menu_data.fps_limiter.into();
                    option_changed = true;
                }
            }
        }
        SelectedMainSetting::ShowFPS => {
            if player_inputs.right.just_pressed {
                if let Some(e) = settings_menu_data.show_fps.enum_next() {
                    settings_menu_data.show_fps = e;
                    fps_overlay_config.enabled = settings_menu_data.show_fps.is_enabled();
                    option_changed = true;
                }
            } else if player_inputs.left.just_pressed {
                if let Some(e) = settings_menu_data.show_fps.enum_prev() {
                    settings_menu_data.show_fps = e;
                    fps_overlay_config.enabled = settings_menu_data.show_fps.is_enabled();
                    option_changed = true;
                }
            }
        }
        SelectedMainSetting::ControllerMapping => {
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
        SelectedMainSetting::ScaleFactor => {
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
    mut query: Query<(Entity, &SelectedMainSettingEntityMarker)>,
    mut tw: TextUiWriter,
    settings_menu_data: Res<SettingsMenuData>,
    game_config: Res<GameConfig>,
    controller_mapping: Res<ControllerMapping>,
    scale_factor: Res<ScaleFactor>,
) {
    query.iter_mut().for_each(|(entity, marker)| {
        let fmt_selected = |tw: &mut TextUiWriter| {
            *tw.text(entity, 0) = (if marker.0 == settings_menu_data.selected_main_setting {
                "▶"
            } else {
                " "
            })
            .into()
        };
        let fmt_larrow = |tw: &mut TextUiWriter, b: bool| {
            *tw.text(entity, 0) = (if b { "<" } else { " " }).into()
        };
        let fmt_rarrow = |tw: &mut TextUiWriter, b: bool| {
            *tw.text(entity, 0) = (if b { ">" } else { " " }).into()
        };
        let fmt_desc = |tw: &mut TextUiWriter, desc: String| *tw.text(entity, 0) = desc;
        match (marker.0, marker.1) {
            (SelectedMainSetting::Tetris, 0) => fmt_selected(&mut tw),
            (SelectedMainSetting::Tetris, 2) => (),
            (SelectedMainSetting::Tetris, 3) => (),
            (SelectedMainSetting::Tetris, 4) => (),
            (SelectedMainSetting::Transition, 0) => fmt_selected(&mut tw),
            (SelectedMainSetting::Transition, 2) => {
                fmt_larrow(&mut tw, game_config.transition.enum_prev().is_some())
            }
            (SelectedMainSetting::Transition, 3) => {
                fmt_desc(&mut tw, game_config.transition.name())
            }
            (SelectedMainSetting::Transition, 4) => {
                fmt_rarrow(&mut tw, game_config.transition.enum_next().is_some())
            }
            (SelectedMainSetting::Linecap, 0) => fmt_selected(&mut tw),
            (SelectedMainSetting::Linecap, 2) => {
                fmt_larrow(&mut tw, game_config.linecap.enum_prev().is_some())
            }
            (SelectedMainSetting::Linecap, 3) => fmt_desc(&mut tw, game_config.linecap.name()),
            (SelectedMainSetting::Linecap, 4) => {
                fmt_rarrow(&mut tw, game_config.linecap.enum_next().is_some())
            }
            (SelectedMainSetting::LinecapLevel, 0) => fmt_selected(&mut tw),
            (SelectedMainSetting::LinecapLevel, 2) => fmt_larrow(
                &mut tw,
                game_config.linecap != crate::game::linecap::Linecap::Off
                    && game_config.linecap_level > 0,
            ),
            (SelectedMainSetting::LinecapLevel, 3) => match game_config.linecap {
                crate::game::linecap::Linecap::Off => fmt_desc(&mut tw, "".into()),
                crate::game::linecap::Linecap::KillScreenX2
                | crate::game::linecap::Linecap::Halt => {
                    fmt_desc(&mut tw, format!("{:02}", game_config.linecap_level.0))
                }
            },
            (SelectedMainSetting::LinecapLevel, 4) => fmt_rarrow(
                &mut tw,
                game_config.linecap != crate::game::linecap::Linecap::Off,
            ),
            (SelectedMainSetting::Gravity, 0) => fmt_selected(&mut tw),
            (SelectedMainSetting::Gravity, 2) => {
                fmt_larrow(&mut tw, game_config.gravity.enum_prev().is_some())
            }
            (SelectedMainSetting::Gravity, 3) => fmt_desc(&mut tw, game_config.gravity.name()),
            (SelectedMainSetting::Gravity, 4) => {
                fmt_rarrow(&mut tw, game_config.gravity.enum_next().is_some())
            }
            (SelectedMainSetting::Seeding, 0) => fmt_selected(&mut tw),
            (SelectedMainSetting::Seeding, 2) => {
                fmt_larrow(&mut tw, game_config.seeding.enum_prev().is_some())
            }
            (SelectedMainSetting::Seeding, 3) => fmt_desc(&mut tw, game_config.seeding.name()),
            (SelectedMainSetting::Seeding, 4) => {
                fmt_rarrow(&mut tw, game_config.seeding.enum_next().is_some())
            }
            (SelectedMainSetting::Seed, 0) => fmt_selected(&mut tw),
            (SelectedMainSetting::Seed, 2) => fmt_larrow(&mut tw, false),
            (SelectedMainSetting::Seed, 3) => match game_config.seeding {
                Seeding::System => fmt_desc(&mut tw, "".into()),
                Seeding::Custom => fmt_desc(&mut tw, game_config.seed.to_string()),
            },
            (SelectedMainSetting::Seed, 4) => fmt_rarrow(&mut tw, false),
            (SelectedMainSetting::Scoring, 0) => fmt_selected(&mut tw),
            (SelectedMainSetting::Scoring, 2) => {
                fmt_larrow(&mut tw, game_config.scoring.enum_prev().is_some())
            }
            (SelectedMainSetting::Scoring, 3) => fmt_desc(&mut tw, game_config.scoring.name()),
            (SelectedMainSetting::Scoring, 4) => {
                fmt_rarrow(&mut tw, game_config.scoring.enum_next().is_some())
            }
            (SelectedMainSetting::Leveling, 0) => fmt_selected(&mut tw),
            (SelectedMainSetting::Leveling, 2) => {
                fmt_larrow(&mut tw, game_config.leveling.enum_prev().is_some())
            }
            (SelectedMainSetting::Leveling, 3) => fmt_desc(&mut tw, game_config.leveling.name()),
            (SelectedMainSetting::Leveling, 4) => {
                fmt_rarrow(&mut tw, game_config.leveling.enum_next().is_some())
            }
            (SelectedMainSetting::TVSystem, 0) => fmt_selected(&mut tw),
            (SelectedMainSetting::TVSystem, 2) => {
                fmt_larrow(&mut tw, game_config.tv_system.enum_prev().is_some())
            }
            (SelectedMainSetting::TVSystem, 3) => fmt_desc(&mut tw, game_config.tv_system.name()),
            (SelectedMainSetting::TVSystem, 4) => {
                fmt_rarrow(&mut tw, game_config.tv_system.enum_next().is_some())
            }
            (SelectedMainSetting::NextPieceHint, 0) => fmt_selected(&mut tw),
            (SelectedMainSetting::NextPieceHint, 2) => {
                fmt_larrow(&mut tw, game_config.next_piece_hint.enum_prev().is_some())
            }
            (SelectedMainSetting::NextPieceHint, 3) => {
                fmt_desc(&mut tw, game_config.next_piece_hint.name())
            }
            (SelectedMainSetting::NextPieceHint, 4) => {
                fmt_rarrow(&mut tw, game_config.next_piece_hint.enum_next().is_some())
            }
            (SelectedMainSetting::Invisible, 0) => fmt_selected(&mut tw),
            (SelectedMainSetting::Invisible, 2) => {
                fmt_larrow(&mut tw, game_config.invisible.enum_prev().is_some())
            }
            (SelectedMainSetting::Invisible, 3) => fmt_desc(&mut tw, game_config.invisible.name()),
            (SelectedMainSetting::Invisible, 4) => {
                fmt_rarrow(&mut tw, game_config.invisible.enum_next().is_some())
            }
            #[cfg(all(not(target_arch = "wasm32"), feature = "fps_limiter"))]
            (SelectedMainSetting::FPSLimiter, 0) => fmt_selected(&mut tw),
            #[cfg(all(not(target_arch = "wasm32"), feature = "fps_limiter"))]
            (SelectedMainSetting::FPSLimiter, 2) => fmt_larrow(
                &mut tw,
                settings_menu_data.fps_limiter.enum_prev().is_some(),
            ),
            #[cfg(all(not(target_arch = "wasm32"), feature = "fps_limiter"))]
            (SelectedMainSetting::FPSLimiter, 3) => {
                fmt_desc(&mut tw, settings_menu_data.fps_limiter.name())
            }
            #[cfg(all(not(target_arch = "wasm32"), feature = "fps_limiter"))]
            (SelectedMainSetting::FPSLimiter, 4) => fmt_rarrow(
                &mut tw,
                settings_menu_data.fps_limiter.enum_next().is_some(),
            ),
            (SelectedMainSetting::ShowFPS, 0) => fmt_selected(&mut tw),
            (SelectedMainSetting::ShowFPS, 2) => {
                fmt_larrow(&mut tw, settings_menu_data.show_fps.enum_prev().is_some())
            }
            (SelectedMainSetting::ShowFPS, 3) => {
                fmt_desc(&mut tw, settings_menu_data.show_fps.name())
            }
            (SelectedMainSetting::ShowFPS, 4) => {
                fmt_rarrow(&mut tw, settings_menu_data.show_fps.enum_next().is_some())
            }
            (SelectedMainSetting::ControllerMapping, 0) => fmt_selected(&mut tw),
            (SelectedMainSetting::ControllerMapping, 2) => {
                fmt_larrow(&mut tw, controller_mapping.enum_prev().is_some())
            }
            (SelectedMainSetting::ControllerMapping, 3) => {
                fmt_desc(&mut tw, controller_mapping.name())
            }
            (SelectedMainSetting::ControllerMapping, 4) => {
                fmt_rarrow(&mut tw, controller_mapping.enum_next().is_some())
            }
            (SelectedMainSetting::ScaleFactor, 0) => fmt_selected(&mut tw),
            (SelectedMainSetting::ScaleFactor, 2) => {
                fmt_larrow(&mut tw, scale_factor.enum_prev().is_some())
            }
            (SelectedMainSetting::ScaleFactor, 3) => fmt_desc(&mut tw, scale_factor.name()),
            (SelectedMainSetting::ScaleFactor, 4) => {
                fmt_rarrow(&mut tw, scale_factor.enum_next().is_some())
            }
            (_, 1) => (),
            (select, idx) => unreachable!("unimplemented option: ({:?}, {})", select, idx),
        }
    });
}
