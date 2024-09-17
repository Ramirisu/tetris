use bevy::{color::palettes::css::WHITE, prelude::*};

use crate::{
    app_state::AppState,
    audio::plugin::PlaySoundEvent,
    controller::Controller,
    game::{drop_speed::DropSpeed, transition::Transition},
    inputs::{ControllerMapping, PlayerInputs},
    level_menu::plugin::LevelMenuData,
    logo::{load_logo_images, TETRIS_BITMAP},
    scale::plugin::ScaleFactor,
    utility::despawn_all,
};

#[cfg(not(target_arch = "wasm32"))]
use bevy::window::WindowMode;

use super::transform::GameOptionMenuTransform;

pub fn setup(app: &mut App) {
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

#[derive(Component)]
struct GameOptionEntityMarker(pub GameOptionMenuSelection);

#[derive(Component)]
struct GameOptionMenuEntityMarker;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum GameOptionMenuSelection {
    #[default]
    Tetris,
    BlankLine0,
    OptionsCategory,
    BlankLine1,
    Transition,
    Linecap,
    DropSpeed,
    ControllerMapping,
    BlankLine2,
    VideoCategory,
    BlankLine3,
    ScaleFactor,
    #[cfg(not(target_arch = "wasm32"))]
    WindowMode,
}

impl GameOptionMenuSelection {
    pub fn iter() -> std::slice::Iter<'static, GameOptionMenuSelection> {
        #[cfg(not(target_arch = "wasm32"))]
        type ArrayType = [GameOptionMenuSelection; 13];
        #[cfg(target_arch = "wasm32")]
        type ArrayType = [GameOptionMenuSelection; 12];
        const STATES: ArrayType = [
            GameOptionMenuSelection::Tetris,
            GameOptionMenuSelection::BlankLine0,
            GameOptionMenuSelection::OptionsCategory,
            GameOptionMenuSelection::BlankLine1,
            GameOptionMenuSelection::Transition,
            GameOptionMenuSelection::Linecap,
            GameOptionMenuSelection::DropSpeed,
            GameOptionMenuSelection::ControllerMapping,
            GameOptionMenuSelection::BlankLine2,
            GameOptionMenuSelection::VideoCategory,
            GameOptionMenuSelection::BlankLine3,
            GameOptionMenuSelection::ScaleFactor,
            #[cfg(not(target_arch = "wasm32"))]
            GameOptionMenuSelection::WindowMode,
        ];
        STATES.iter()
    }
}

#[derive(Resource)]
struct GameOptionMenuData {
    selection: GameOptionMenuSelection,
    transition: Transition,
    lv39_linecap: bool,
    drop_speed: DropSpeed,
    #[cfg(not(target_arch = "wasm32"))]
    window_mode: WindowMode,
}

impl GameOptionMenuData {
    pub fn new() -> Self {
        Self {
            selection: GameOptionMenuSelection::default(),
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
) {
    query.iter_mut().for_each(|(mut text, marker)| {
        enum NameKind {
            Category,
            Option,
        }
        let fname = |name, kind: NameKind| -> String {
            match kind {
                NameKind::Category => format!("{:25} ", name),
                NameKind::Option => {
                    let s = if marker.0 == game_option_menu_data.selection {
                        ">>"
                    } else {
                        ""
                    };
                    format!("{:>4} {:20} ", s, name)
                }
            }
        };
        let fopt = |name, l, r| -> String {
            let l = if l { "<" } else { "" };
            let r = if r { ">" } else { "" };
            format!("{:1} {:12} {:1}", l, name, r)
        };
        match marker.0 {
            GameOptionMenuSelection::Tetris => {
                text.sections[0].value = fname("TETRIS", NameKind::Option);
                text.sections[1].value = fopt("", false, false);
            }
            GameOptionMenuSelection::BlankLine0 => {
                text.sections[0].value = fname("", NameKind::Category);
                text.sections[1].value = fopt("", false, false);
            }
            GameOptionMenuSelection::OptionsCategory => {
                text.sections[0].value = fname("# OPTIONS", NameKind::Category);
                text.sections[1].value = fopt("", false, false);
            }
            GameOptionMenuSelection::BlankLine1 => {
                text.sections[0].value = fname("", NameKind::Category);
                text.sections[1].value = fopt("", false, false);
            }
            GameOptionMenuSelection::Transition => {
                text.sections[0].value = fname("TRANSITION", NameKind::Option);
                match game_option_menu_data.transition {
                    Transition::Default => text.sections[1].value = fopt("DEFAULT", false, true),
                    Transition::Every10Lines => {
                        text.sections[1].value = fopt("10 LINES", true, true)
                    }
                    Transition::Every4Lines => {
                        text.sections[1].value = fopt(" 4 LINES", true, false)
                    }
                };
            }
            GameOptionMenuSelection::Linecap => {
                text.sections[0].value = fname("LV39 LINECAP", NameKind::Option);
                if game_option_menu_data.lv39_linecap {
                    text.sections[1].value = fopt("ON", true, false);
                } else {
                    text.sections[1].value = fopt("OFF", false, true);
                }
            }
            GameOptionMenuSelection::DropSpeed => {
                text.sections[0].value = fname("DROPSPEED", NameKind::Option);
                match game_option_menu_data.drop_speed {
                    DropSpeed::Level => text.sections[1].value = fopt("LEVEL", false, true),
                    DropSpeed::Locked => text.sections[1].value = fopt("LOCKED", true, false),
                };
            }
            GameOptionMenuSelection::ControllerMapping => {
                text.sections[0].value = fname("CONTROLLER MAPPING", NameKind::Option);
                match *controller_mapping {
                    ControllerMapping::MappingA => {
                        text.sections[1].value = fopt("MAPPING A", false, true)
                    }
                    ControllerMapping::MappingB => {
                        text.sections[1].value = fopt("MAPPING B", true, false)
                    }
                };
            }
            GameOptionMenuSelection::BlankLine2 => {
                text.sections[0].value = fname("", NameKind::Category);
                text.sections[1].value = fopt("", false, false);
            }
            GameOptionMenuSelection::VideoCategory => {
                text.sections[0].value = fname("# VIDEO OPTIONS", NameKind::Category);
                text.sections[1].value = fopt("", false, false);
            }
            GameOptionMenuSelection::BlankLine3 => {
                text.sections[0].value = fname("", NameKind::Category);
                text.sections[1].value = fopt("", false, false);
            }
            GameOptionMenuSelection::ScaleFactor => {
                text.sections[0].value = fname("SCALE FACTOR", NameKind::Option);
                match *scale_factor {
                    ScaleFactor::S720 => text.sections[1].value = fopt("0.66 (720P)", false, true),
                    ScaleFactor::S1080 => text.sections[1].value = fopt("1.00 (1080P)", true, true),
                    ScaleFactor::S1440 => text.sections[1].value = fopt("1.33 (1440P)", true, true),
                    ScaleFactor::S1800 => text.sections[1].value = fopt("1.66 (1800P)", true, true),
                    ScaleFactor::S2160 => text.sections[1].value = fopt("2.00 (2160P)", true, true),
                    ScaleFactor::S3240 => text.sections[1].value = fopt("3.00 (3240P)", true, true),
                    ScaleFactor::S4320 => {
                        text.sections[1].value = fopt("4.00 (4320P)", true, false)
                    }
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            GameOptionMenuSelection::WindowMode => {
                text.sections[0].value = fname("WINDOW MODE", NameKind::Option);
                match game_option_menu_data.window_mode {
                    WindowMode::Windowed => {
                        text.sections[1].value = fopt("WINDOWED", false, true);
                    }
                    WindowMode::BorderlessFullscreen => {
                        text.sections[1].value = fopt("BORDERLESS", true, false);
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
                level_menu_data.config.transition = game_option_menu_data.transition;
                level_menu_data.config.lv39_linecap = game_option_menu_data.lv39_linecap;
                level_menu_data.config.drop_speed = game_option_menu_data.drop_speed;
                e_play_sound.send(PlaySoundEvent::StartGame);
                app_state.set(AppState::LevelMenu);
            }
        }
        GameOptionMenuSelection::BlankLine0 => (),
        GameOptionMenuSelection::OptionsCategory => (),
        GameOptionMenuSelection::BlankLine1 => (),
        GameOptionMenuSelection::Transition => {
            if player_inputs.up.0 {
                game_option_menu_data.selection = GameOptionMenuSelection::Tetris;
                selection_changed = true;
            } else if player_inputs.down.0 {
                game_option_menu_data.selection = GameOptionMenuSelection::Linecap;
                selection_changed = true;
            }
            match game_option_menu_data.transition {
                Transition::Default => {
                    if player_inputs.right.0 {
                        game_option_menu_data.transition = Transition::Every10Lines;
                        option_changed = true;
                    }
                }
                Transition::Every10Lines => {
                    if player_inputs.right.0 {
                        game_option_menu_data.transition = Transition::Every4Lines;
                        option_changed = true;
                    } else if player_inputs.left.0 {
                        game_option_menu_data.transition = Transition::Default;
                        option_changed = true;
                    }
                }
                Transition::Every4Lines => {
                    if player_inputs.left.0 {
                        game_option_menu_data.transition = Transition::Every10Lines;
                        option_changed = true;
                    }
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
                game_option_menu_data.lv39_linecap = true;
                option_changed = true;
            } else if player_inputs.left.0 {
                game_option_menu_data.lv39_linecap = false;
                option_changed = true;
            }
        }
        GameOptionMenuSelection::DropSpeed => {
            if player_inputs.up.0 {
                game_option_menu_data.selection = GameOptionMenuSelection::Linecap;
                selection_changed = true;
            } else if player_inputs.down.0 {
                game_option_menu_data.selection = GameOptionMenuSelection::ControllerMapping;
                selection_changed = true;
            }
            if player_inputs.right.0 {
                game_option_menu_data.drop_speed = DropSpeed::Locked;
                option_changed = true;
            } else if player_inputs.left.0 {
                game_option_menu_data.drop_speed = DropSpeed::Level;
                option_changed = true;
            }
        }
        GameOptionMenuSelection::ControllerMapping => {
            if player_inputs.up.0 {
                game_option_menu_data.selection = GameOptionMenuSelection::DropSpeed;
                selection_changed = true;
            } else if player_inputs.down.0 {
                game_option_menu_data.selection = GameOptionMenuSelection::ScaleFactor;
                selection_changed = true;
            }
            if player_inputs.right.0 {
                *controller_mapping = ControllerMapping::MappingB;
                option_changed = true;
            } else if player_inputs.left.0 {
                *controller_mapping = ControllerMapping::MappingA;
                option_changed = true;
            }
        }
        GameOptionMenuSelection::BlankLine2 => (),
        GameOptionMenuSelection::VideoCategory => (),
        GameOptionMenuSelection::BlankLine3 => (),
        GameOptionMenuSelection::ScaleFactor => {
            if player_inputs.up.0 {
                game_option_menu_data.selection = GameOptionMenuSelection::ControllerMapping;
                selection_changed = true;
            } else if player_inputs.down.0 {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    game_option_menu_data.selection = GameOptionMenuSelection::WindowMode;
                }
                #[cfg(target_arch = "wasm32")]
                {
                    game_option_menu_data.selection = GameOptionMenuSelection::Tetris;
                }
                selection_changed = true;
            }

            if player_inputs.right.0 {
                if let Some(_) = scale_factor.next() {
                    scale_changed = true;
                }
            } else if player_inputs.left.0 {
                if let Some(_) = scale_factor.prev() {
                    scale_changed = true;
                }
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        GameOptionMenuSelection::WindowMode => {
            if player_inputs.up.0 {
                game_option_menu_data.selection = GameOptionMenuSelection::ScaleFactor;
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
                    if player_inputs.left.0 {
                        game_option_menu_data.window_mode = WindowMode::Windowed;
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
