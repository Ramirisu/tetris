use bevy::{
    color::palettes::css::{BLACK, GREEN, RED, WHITE, YELLOW},
    prelude::*,
};

use crate::{app_state::AppState, controller::Controller, utility::despawn_all};

use super::{
    board::Board,
    palette::{get_block_image, get_default_block_image},
    piece::PieceShape,
    render::RenderConfig,
    tick::{duration_to_ticks, EntryDelayTick, FallTick, LineClearTick},
    timer::{DelayAutoShiftTimer, GameTimer, PressDownTimer},
};

pub fn setup(app: &mut App) {
    app.insert_resource(PlayerData::default())
        .init_state::<PlayerState>()
        .add_event::<PlaySoundEvent>()
        .add_systems(
            OnEnter(AppState::Game),
            (load_audio_assets, load_block_image_assets, setup_screen).chain(),
        )
        .add_systems(
            OnExit(AppState::Game),
            (
                despawn_all::<GameEntityMarker>,
                unload_block_image_assets,
                unload_audio_assets,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                (
                    state_game_running::tick_system,
                    state_game_running::handle_input_system,
                    state_game_running::curr_piece_fall_system,
                    update_statistic_system,
                )
                    .chain()
                    .run_if(in_state(PlayerState::GameRunning)),
                (state_game_line_clear::tick_system, update_statistic_system)
                    .chain()
                    .run_if(in_state(PlayerState::GameLineClear)),
                state_game_update_assets::update_block_image_assets
                    .run_if(in_state(PlayerState::GameUpdateBlockImageAssets)),
                state_game_entry_delay::tick_system.run_if(in_state(PlayerState::GameEntryDelay)),
                state_game_pause::handle_input_system.run_if(in_state(PlayerState::GamePause)),
                state_game_over::handle_input_system.run_if(in_state(PlayerState::GameOver)),
                play_sound_system,
            )
                .run_if(in_state(AppState::Game)),
        );
}

#[derive(Resource)]
struct AudioAssets {
    move_curr_piece: Handle<AudioSource>,
    rotate_curr_piece: Handle<AudioSource>,
    lock_curr_piece: Handle<AudioSource>,
    line_clear: Handle<AudioSource>,
    tetris_clear: Handle<AudioSource>,
    level_up: Handle<AudioSource>,
    game_over: Handle<AudioSource>,
}

#[derive(Resource)]
struct BlockImageAssets {
    images: Vec<Handle<Image>>,
    default: Handle<Image>,
}

impl BlockImageAssets {
    pub fn get_image(&self, shape: PieceShape) -> Handle<Image> {
        self.images[shape as usize].clone()
    }

    pub fn get_default(&self) -> Handle<Image> {
        self.default.clone()
    }

    pub fn get_image_or_default(&self, shape: Option<PieceShape>) -> Handle<Image> {
        if let Some(shape) = shape {
            self.get_image(shape)
        } else {
            self.get_default()
        }
    }
}

#[derive(Component)]
struct GameEntityMarker;

#[derive(Component, Clone, Copy)]
struct BoardBlockEntityMarker(usize, usize);

impl Into<(usize, usize)> for &BoardBlockEntityMarker {
    fn into(self) -> (usize, usize) {
        (self.0, self.1)
    }
}

#[derive(Component)]
struct BoardCoverEntityMarker;

#[derive(Component)]
struct LinesEntityMarker;

#[derive(Component)]
struct ScoreEntityMarker;

#[derive(Component)]
struct LevelEntityMarker;

#[derive(Component)]
struct DASEntityMarker;

#[derive(Component)]
struct TetrisEntityMarker;

#[derive(Component)]
struct CurrPieceEntityMarker;

#[derive(Component)]
struct NextPieceEntityMarker;

#[derive(Component)]
struct NextPieceSlotCoverEntityMarker;

#[derive(Event)]
enum PlaySoundEvent {
    MoveCurrPiece,
    RotateCurrPiece,
    LockCurrPiece,
    LineClear,
    TetrisClear,
    LevelUp,
    GameOver,
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum PlayerState {
    #[default]
    GameRunning,
    GameLineClear,
    GameUpdateBlockImageAssets,
    GameEntryDelay,
    GamePause,
    GameOver,
}

#[derive(Resource)]
pub struct PlayerData {
    rc: RenderConfig,
    board: Board,
    game_timer: GameTimer,
    can_press_down: bool,
    press_down_timer: PressDownTimer,
    das_timer: DelayAutoShiftTimer,
    fall_tick: FallTick,
    line_clear_tick: LineClearTick,
    line_clear_rows: Vec<usize>,
    line_clear_phase: state_game_line_clear::LineClearPhase,
    entry_delay_tick: EntryDelayTick,
}

impl PlayerData {
    pub fn new(start_level: usize) -> Self {
        Self {
            rc: RenderConfig::default(),
            board: Board::new(start_level),
            game_timer: GameTimer::default(),
            can_press_down: false,
            press_down_timer: PressDownTimer::default(),
            das_timer: DelayAutoShiftTimer::default(),
            fall_tick: FallTick::new(start_level, true),
            line_clear_tick: LineClearTick::default(),
            line_clear_rows: default(),
            line_clear_phase: state_game_line_clear::LineClearPhase::default(),
            entry_delay_tick: EntryDelayTick::default(),
        }
    }
}

impl Default for PlayerData {
    fn default() -> Self {
        Self::new(0)
    }
}

fn load_audio_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(AudioAssets {
        move_curr_piece: asset_server.load("sound/sfx04.wav"),
        rotate_curr_piece: asset_server.load("sound/sfx06.wav"),
        lock_curr_piece: asset_server.load("sound/sfx08.wav"),
        line_clear: asset_server.load("sound/sfx11.wav"),
        tetris_clear: asset_server.load("sound/sfx19.wav"),
        level_up: asset_server.load("sound/sfx07.wav"),
        game_over: asset_server.load("sound/sfx14.wav"),
    });
}

fn unload_audio_assets(mut commands: Commands) {
    commands.remove_resource::<AudioAssets>();
}

fn load_block_image_assets(
    mut commands: Commands,
    mut image_assets: ResMut<Assets<Image>>,
    player_data: Res<PlayerData>,
) {
    commands.insert_resource(BlockImageAssets {
        images: PieceShape::iter()
            .map(|shape| image_assets.add(get_block_image(*shape, player_data.board.level())))
            .collect(),
        default: image_assets.add(get_default_block_image()),
    });
}

fn unload_block_image_assets(mut commands: Commands) {
    commands.remove_resource::<BlockImageAssets>();
}

fn setup_screen(
    mut commands: Commands,
    player_data: ResMut<PlayerData>,
    block_image_assets: Res<BlockImageAssets>,
) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(player_data.rc.board_background_translation()),
            sprite: Sprite {
                color: RED.into(),
                custom_size: Some(player_data.rc.board_background_size()),
                ..default()
            },
            ..default()
        },
        GameEntityMarker,
    ));
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(player_data.rc.board_translation()),
            sprite: Sprite {
                color: BLACK.into(),
                custom_size: Some(player_data.rc.board_size()),
                ..default()
            },
            ..default()
        },
        GameEntityMarker,
    ));

    for y in 0..Board::BOARD_ROWS {
        for x in 0..Board::BOARD_COLS {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_translation(
                        player_data.rc.board_block_translation(x as i32, y as i32),
                    ),
                    sprite: Sprite {
                        custom_size: Some(player_data.rc.visible_block_size()),
                        ..default()
                    },
                    texture: block_image_assets.get_default(),
                    ..default()
                },
                GameEntityMarker,
                BoardBlockEntityMarker(x, y),
            ));
        }
    }

    commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_translation(player_data.rc.board_cover_translation()),
                sprite: Sprite {
                    color: BLACK.into(),
                    custom_size: Some(player_data.rc.board_size()),
                    ..default()
                },
                visibility: Visibility::Hidden,
                ..default()
            },
            GameEntityMarker,
            BoardCoverEntityMarker,
        ))
        .with_children(|parent| {
            parent.spawn(Text2dBundle {
                text: Text::from_section(
                    "PRESS START\nTO CONTINUE",
                    TextStyle {
                        font_size: player_data.rc.unit(),
                        color: WHITE.into(),
                        ..default()
                    },
                ),
                transform: Transform::from_translation(player_data.rc.board_cover_translation()),
                ..default()
            });
        });

    commands.spawn((
        Text2dBundle {
            text: Text::from_sections(vec![
                TextSection {
                    value: "LINES\n".into(),
                    style: TextStyle {
                        font_size: player_data.rc.unit(),
                        color: WHITE.into(),
                        ..default()
                    },
                    ..default()
                },
                TextSection::from_style(TextStyle {
                    font_size: player_data.rc.unit() * 2.0,
                    color: WHITE.into(),
                    ..default()
                }),
            ])
            .with_justify(JustifyText::Center),
            transform: Transform::from_translation(player_data.rc.lines_translation()),
            ..default()
        },
        GameEntityMarker,
        LinesEntityMarker,
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_sections(vec![
                TextSection {
                    value: "SCORE\n".into(),
                    style: TextStyle {
                        font_size: player_data.rc.unit(),
                        color: WHITE.into(),
                        ..default()
                    },
                    ..default()
                },
                TextSection::from_style(TextStyle {
                    font_size: player_data.rc.unit() * 2.0,
                    color: WHITE.into(),
                    ..default()
                }),
            ])
            .with_justify(JustifyText::Center),
            transform: Transform::from_translation(player_data.rc.score_translation()),
            ..default()
        },
        GameEntityMarker,
        ScoreEntityMarker,
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_sections(vec![
                TextSection {
                    value: "LEVEL ".into(),
                    style: TextStyle {
                        font_size: player_data.rc.unit(),
                        color: WHITE.into(),
                        ..default()
                    },
                    ..default()
                },
                TextSection::from_style(TextStyle {
                    font_size: player_data.rc.unit() * 2.0,
                    color: WHITE.into(),
                    ..default()
                }),
                TextSection::from_style(TextStyle {
                    font_size: player_data.rc.unit() / 1.5,
                    color: WHITE.into(),
                    ..default()
                }),
            ]),
            transform: Transform::from_translation(player_data.rc.level_translation()),
            ..default()
        },
        GameEntityMarker,
        LevelEntityMarker,
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_sections(vec![
                TextSection {
                    value: "BRN ".into(),
                    style: TextStyle {
                        font_size: player_data.rc.unit(),
                        color: WHITE.into(),
                        ..default()
                    },
                    ..default()
                },
                TextSection::from_style(TextStyle {
                    font_size: player_data.rc.unit(),
                    color: WHITE.into(),
                    ..default()
                }),
                TextSection {
                    value: " 1X ".into(),
                    style: TextStyle {
                        font_size: player_data.rc.unit(),
                        color: WHITE.into(),
                        ..default()
                    },
                    ..default()
                },
                TextSection::from_style(TextStyle {
                    font_size: player_data.rc.unit(),
                    color: WHITE.into(),
                    ..default()
                }),
                TextSection {
                    value: " 2X ".into(),
                    style: TextStyle {
                        font_size: player_data.rc.unit(),
                        color: WHITE.into(),
                        ..default()
                    },
                    ..default()
                },
                TextSection::from_style(TextStyle {
                    font_size: player_data.rc.unit(),
                    color: WHITE.into(),
                    ..default()
                }),
                TextSection {
                    value: " 3X ".into(),
                    style: TextStyle {
                        font_size: player_data.rc.unit(),
                        color: WHITE.into(),
                        ..default()
                    },
                    ..default()
                },
                TextSection::from_style(TextStyle {
                    font_size: player_data.rc.unit(),
                    color: WHITE.into(),
                    ..default()
                }),
                TextSection {
                    value: "TRT ".into(),
                    style: TextStyle {
                        font_size: player_data.rc.unit(),
                        color: WHITE.into(),
                        ..default()
                    },
                    ..default()
                },
                TextSection::from_style(TextStyle {
                    font_size: player_data.rc.unit(),
                    color: WHITE.into(),
                    ..default()
                }),
                TextSection {
                    value: "TRT ".into(),
                    style: TextStyle {
                        font_size: player_data.rc.unit(),
                        color: WHITE.into(),
                        ..default()
                    },
                    ..default()
                },
                TextSection::from_style(TextStyle {
                    font_size: player_data.rc.unit(),
                    color: WHITE.into(),
                    ..default()
                }),
                TextSection {
                    value: "DRT ".into(),
                    style: TextStyle {
                        font_size: player_data.rc.unit(),
                        color: WHITE.into(),
                        ..default()
                    },
                    ..default()
                },
                TextSection::from_style(TextStyle {
                    font_size: player_data.rc.unit(),
                    color: WHITE.into(),
                    ..default()
                }),
            ]),
            transform: Transform::from_translation(player_data.rc.tetris_translation()),
            ..default()
        },
        GameEntityMarker,
        TetrisEntityMarker,
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_sections(vec![
                TextSection {
                    value: "DAS ".into(),
                    style: TextStyle {
                        font_size: player_data.rc.unit(),
                        color: WHITE.into(),
                        ..default()
                    },
                    ..default()
                },
                TextSection::from_style(TextStyle {
                    font_size: player_data.rc.unit(),
                    color: WHITE.into(),
                    ..default()
                }),
            ]),
            transform: Transform::from_translation(player_data.rc.das_translation()),
            ..default()
        },
        GameEntityMarker,
        DASEntityMarker,
    ));

    player_data
        .board
        .get_curr_piece_blocks()
        .iter()
        .for_each(|blk| {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_translation(
                        player_data.rc.curr_piece_translation(blk.0, blk.1),
                    ),
                    sprite: Sprite {
                        custom_size: Some(player_data.rc.visible_block_size()),
                        ..default()
                    },
                    texture: block_image_assets
                        .get_image(player_data.board.get_curr_piece().shape()),
                    ..default()
                },
                GameEntityMarker,
                CurrPieceEntityMarker,
            ));
        });

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(
                player_data.rc.next_piece_slot_background_translation(),
            ),
            sprite: Sprite {
                color: RED.into(),
                custom_size: Some(player_data.rc.next_piece_slot_background_size()),
                ..default()
            },
            ..default()
        },
        GameEntityMarker,
    ));
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(player_data.rc.next_piece_slot_translation()),
            sprite: Sprite {
                color: BLACK.into(),
                custom_size: Some(player_data.rc.next_piece_slot_size()),
                ..default()
            },
            ..default()
        },
        GameEntityMarker,
    ));
    player_data
        .board
        .get_next_piece()
        .to_blocks()
        .iter()
        .for_each(|blk| {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_translation(
                        player_data.rc.next_piece_translation(blk.0, blk.1),
                    ),
                    sprite: Sprite {
                        custom_size: Some(player_data.rc.visible_block_size()),
                        ..default()
                    },
                    texture: block_image_assets
                        .get_image(player_data.board.get_next_piece().shape()),
                    ..default()
                },
                GameEntityMarker,
                NextPieceEntityMarker,
            ));
        });
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(
                player_data.rc.next_piece_slot_cover_translation(),
            ),
            sprite: Sprite {
                color: BLACK.into(),
                custom_size: Some(player_data.rc.next_piece_slot_size()),
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        },
        GameEntityMarker,
        NextPieceSlotCoverEntityMarker,
    ));
}

fn update_statistic_system(
    mut query: ParamSet<(
        Query<&mut Text, With<LinesEntityMarker>>,
        Query<&mut Text, With<ScoreEntityMarker>>,
        Query<&mut Text, With<LevelEntityMarker>>,
        Query<&mut Text, With<TetrisEntityMarker>>,
        Query<&mut Text, With<DASEntityMarker>>,
    )>,
    player_data: ResMut<PlayerData>,
) {
    if let Ok(mut text) = query.p0().get_single_mut() {
        text.sections[1].value = format!("{:04}", player_data.board.lines());
    }
    if let Ok(mut text) = query.p1().get_single_mut() {
        text.sections[1].value = format!("{:07}", player_data.board.score());
    }
    if let Ok(mut text) = query.p2().get_single_mut() {
        text.sections[1].value = format!("{:02}", player_data.board.level());
        text.sections[2].value = format!(" {:02}", player_data.board.start_level());
    }
    if let Ok(mut text) = query.p3().get_single_mut() {
        text.sections[1].value = format!("{:4}\n", player_data.board.burned_lines());
        text.sections[3].value = format!("{:4}\n", player_data.board.single());
        text.sections[5].value = format!("{:4}\n", player_data.board.double());
        text.sections[7].value = format!("{:4}\n", player_data.board.triple());
        text.sections[9].value = format!("{:4}\n", player_data.board.tetris());

        let rate = (player_data.board.tetris_rate() * 100.0).round() as usize;
        text.sections[11].value = format!("{:3}%\n", rate);
        if rate >= 80 {
            text.sections[11].style.color = GREEN.into();
        } else if rate >= 50 {
            text.sections[11].style.color = YELLOW.into();
        } else {
            text.sections[11].style.color = RED.into();
        }
        let drought = player_data.board.drought();
        text.sections[13].value = format!("{:4}\n", drought);
        if drought >= 14 {
            text.sections[13].style.color = RED.into();
        } else if drought >= 7 {
            text.sections[13].style.color = YELLOW.into();
        } else {
            text.sections[13].style.color = GREEN.into();
        }
    }
    if let Ok(mut text) = query.p4().get_single_mut() {
        let ticks = duration_to_ticks(player_data.das_timer.duration());
        text.sections[1].value = format!("{:02}", ticks);
        if ticks >= 10 {
            text.sections[1].style.color = GREEN.into();
        } else {
            text.sections[1].style.color = RED.into();
        }
    }
}

fn play_sound_system(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    mut event_reader: EventReader<PlaySoundEvent>,
) {
    for event in event_reader.read() {
        let audio = match event {
            PlaySoundEvent::MoveCurrPiece => audio_assets.move_curr_piece.clone(),
            PlaySoundEvent::RotateCurrPiece => audio_assets.rotate_curr_piece.clone(),
            PlaySoundEvent::LockCurrPiece => audio_assets.lock_curr_piece.clone(),
            PlaySoundEvent::LineClear => audio_assets.line_clear.clone(),
            PlaySoundEvent::TetrisClear => audio_assets.tetris_clear.clone(),
            PlaySoundEvent::LevelUp => audio_assets.level_up.clone(),
            PlaySoundEvent::GameOver => audio_assets.game_over.clone(),
        };
        commands.spawn(AudioBundle {
            source: audio,
            settings: PlaybackSettings::DESPAWN,
        });
    }
}

mod state_game_running {
    use state_game_line_clear::LineClearPhase;

    use super::*;

    pub(super) fn tick_system(time: Res<Time>, mut player_data: ResMut<PlayerData>) {
        player_data.game_timer.tick(time.delta());
        player_data.press_down_timer.tick(time.delta());
    }

    pub struct GameRunningInputs {
        left: (bool, bool),  // (just_pressed, pressed)
        right: (bool, bool), // (just_pressed, pressed)
        down: (bool, bool),  // (just_pressed, pressed)
        rotate_clockwise: bool,
        rotate_counter_clockwise: bool,
        start: bool,
    }

    impl std::ops::BitOrAssign for GameRunningInputs {
        fn bitor_assign(&mut self, rhs: Self) {
            self.left.0 |= rhs.left.0;
            self.left.1 |= rhs.left.1;
            self.right.0 |= rhs.right.0;
            self.right.1 |= rhs.right.1;
            self.down.0 |= rhs.down.0;
            self.down.1 |= rhs.down.1;
            self.rotate_clockwise |= rhs.rotate_clockwise;
            self.rotate_counter_clockwise |= rhs.rotate_counter_clockwise;
            self.start |= rhs.start;
        }
    }

    pub(super) fn handle_input_system(
        time: Res<Time>,
        keys: Res<ButtonInput<KeyCode>>,
        buttons: Res<ButtonInput<GamepadButton>>,
        controller: Res<Controller>,
        mut query: ParamSet<(
            Query<(&mut Transform, &mut Handle<Image>), With<CurrPieceEntityMarker>>,
            Query<&mut Visibility, With<BoardCoverEntityMarker>>,
            Query<&mut Visibility, With<NextPieceSlotCoverEntityMarker>>,
        )>,
        mut e_play_sound: EventWriter<PlaySoundEvent>,
        mut player_data: ResMut<PlayerData>,
        mut player_state: ResMut<NextState<PlayerState>>,
        block_image_assets: Res<BlockImageAssets>,
    ) {
        let mut inputs = GameRunningInputs {
            left: (
                keys.just_pressed(KeyCode::KeyA),
                keys.pressed(KeyCode::KeyA),
            ),
            right: (
                keys.just_pressed(KeyCode::KeyD),
                keys.pressed(KeyCode::KeyD),
            ),
            down: (
                keys.just_pressed(KeyCode::KeyS),
                keys.pressed(KeyCode::KeyS),
            ),
            rotate_clockwise: keys.just_pressed(KeyCode::Period),
            rotate_counter_clockwise: keys.just_pressed(KeyCode::Comma),
            start: keys.just_pressed(KeyCode::Enter),
        };

        if let Some(gamepad) = controller.gamepad {
            inputs |= GameRunningInputs {
                left: (
                    buttons.just_pressed(GamepadButton {
                        gamepad,
                        button_type: GamepadButtonType::DPadLeft,
                    }),
                    buttons.pressed(GamepadButton {
                        gamepad,
                        button_type: GamepadButtonType::DPadLeft,
                    }),
                ),
                right: (
                    buttons.just_pressed(GamepadButton {
                        gamepad,
                        button_type: GamepadButtonType::DPadRight,
                    }),
                    buttons.pressed(GamepadButton {
                        gamepad,
                        button_type: GamepadButtonType::DPadRight,
                    }),
                ),
                down: (
                    buttons.just_pressed(GamepadButton {
                        gamepad,
                        button_type: GamepadButtonType::DPadDown,
                    }),
                    buttons.pressed(GamepadButton {
                        gamepad,
                        button_type: GamepadButtonType::DPadDown,
                    }),
                ),
                rotate_clockwise: buttons.just_pressed(GamepadButton {
                    gamepad,
                    button_type: GamepadButtonType::South,
                }),
                rotate_counter_clockwise: buttons.just_pressed(GamepadButton {
                    gamepad,
                    button_type: GamepadButtonType::West,
                }),
                start: buttons.just_pressed(GamepadButton {
                    gamepad,
                    button_type: GamepadButtonType::Start,
                }),
            };
        }

        if inputs.start {
            *query.p1().single_mut() = Visibility::Inherited;
            *query.p2().single_mut() = Visibility::Inherited;
            player_state.set(PlayerState::GamePause);
            return;
        }

        let (moved, lr_moved, rotated) = handle_input(inputs, &time, &mut player_data);
        if moved {
            std::iter::zip(
                query.p0().iter_mut(),
                player_data.board.get_curr_piece_blocks(),
            )
            .for_each(|((mut transform, mut image), blk)| {
                *image = block_image_assets.get_image(player_data.board.get_curr_piece().shape());
                transform.translation = player_data.rc.curr_piece_translation(blk.0, blk.1);
            });
        }
        if lr_moved {
            e_play_sound.send(PlaySoundEvent::MoveCurrPiece);
        }
        if rotated {
            e_play_sound.send(PlaySoundEvent::RotateCurrPiece);
        }
    }

    fn handle_input(
        inputs: GameRunningInputs,
        time: &Time,
        player_data: &mut PlayerData,
    ) -> (bool, bool, bool) {
        let mut moved = false;
        let mut lr_moved = false;
        let mut rotated = false;

        if player_data.can_press_down {
            if inputs.down.1 {
                if player_data.press_down_timer.commit() {
                    moved |= player_data.board.move_piece_down();
                }
            } else {
                player_data.can_press_down = false;
            }
        } else if inputs.down.0 {
            player_data.can_press_down = true;
            player_data.game_timer.reset();
            player_data.fall_tick = FallTick::new(player_data.board.level(), false);
            player_data.press_down_timer.reset();
        }

        if !inputs.down.1 {
            player_data.press_down_timer.reset();

            if inputs.left.0 || inputs.right.0 {
                player_data.das_timer.reset();
                match (inputs.left.0, inputs.right.0) {
                    (true, false) => lr_moved |= player_data.board.move_piece_left(),
                    (false, true) => lr_moved |= player_data.board.move_piece_right(),
                    _ => (),
                }
            } else {
                match (inputs.left.1, inputs.right.1) {
                    (true, true) => player_data.das_timer.tick(time.delta()),
                    (true, false) => {
                        player_data.das_timer.tick(time.delta());
                        if !player_data.board.is_left_movable() {
                            player_data.das_timer.reset_max();
                        } else if player_data.das_timer.commit() {
                            lr_moved |= player_data.board.move_piece_left();
                        }
                    }
                    (false, true) => {
                        player_data.das_timer.tick(time.delta());
                        if !player_data.board.is_right_movable() {
                            player_data.das_timer.reset_max();
                        } else if player_data.das_timer.commit() {
                            lr_moved |= player_data.board.move_piece_right();
                        }
                    }
                    _ => (),
                }
            }
        }

        if inputs.rotate_clockwise {
            rotated |= player_data.board.rotate_piece_clockwise();
        }
        if inputs.rotate_counter_clockwise {
            rotated |= player_data.board.rotate_piece_counter_clockwise();
        }

        (moved | lr_moved | rotated, lr_moved, rotated)
    }

    pub(super) fn curr_piece_fall_system(
        mut query: ParamSet<(
            Query<(&mut Handle<Image>, &BoardBlockEntityMarker)>,
            Query<(&mut Transform, &mut Handle<Image>), With<CurrPieceEntityMarker>>,
        )>,
        mut e_play_sound: EventWriter<PlaySoundEvent>,
        mut player_data: ResMut<PlayerData>,
        mut player_state: ResMut<NextState<PlayerState>>,
        block_image_assets: Res<BlockImageAssets>,
    ) {
        let threshold = player_data.fall_tick.threshold();
        if player_data.game_timer.commit(threshold) {
            player_data.fall_tick = FallTick::new(player_data.board.level(), false);

            if player_data.board.move_piece_down() {
                std::iter::zip(
                    query.p1().iter_mut(),
                    player_data.board.get_curr_piece_blocks(),
                )
                .for_each(|((mut transform, mut image), blk)| {
                    *image =
                        block_image_assets.get_image(player_data.board.get_curr_piece().shape());
                    transform.translation = player_data.rc.curr_piece_translation(blk.0, blk.1);
                });
            } else if !player_data.board.is_curr_position_valid() {
                e_play_sound.send(PlaySoundEvent::GameOver);
                player_state.set(PlayerState::GameOver);
            } else {
                player_data.can_press_down = false; // keep pressing down will not affect next piece

                let min_y = player_data
                    .board
                    .get_curr_piece_blocks()
                    .iter()
                    .fold(19, |acc, blk| acc.min(blk.1 as u64));
                player_data.entry_delay_tick = EntryDelayTick::new(min_y);

                player_data.board.lock_curr_piece();
                query.p1().iter_mut().for_each(|(mut transform, _)| {
                    // make invisible
                    transform.translation.z = player_data.rc.board_translation().z - 1.0;
                });

                query.p0().iter_mut().for_each(|(mut image, coordinate)| {
                    *image = block_image_assets.get_image_or_default(
                        player_data
                            .board
                            .block(coordinate.0 as i32, coordinate.1 as i32),
                    );
                });

                let lines = player_data.board.get_line_clear_indexes();
                match lines.len() {
                    0 => {
                        e_play_sound.send(PlaySoundEvent::LockCurrPiece);
                    }
                    1 | 2 | 3 => {
                        e_play_sound.send(PlaySoundEvent::LineClear);
                    }
                    4 => {
                        e_play_sound.send(PlaySoundEvent::TetrisClear);
                    }
                    _ => (),
                }
                if lines.len() > 0 {
                    player_data.line_clear_tick = LineClearTick::new((Board::BOARD_COLS + 1) / 2);
                    player_data.line_clear_rows = lines;
                    player_data.line_clear_phase = LineClearPhase::new();
                    player_state.set(PlayerState::GameLineClear);
                } else {
                    player_state.set(PlayerState::GameEntryDelay);
                }
            }
        }
    }
}

mod state_game_line_clear {
    use super::*;

    pub(super) struct LineClearPhase {
        cols: Option<(usize, usize)>, // (left, right)
    }

    impl LineClearPhase {
        pub fn new() -> Self {
            const COLS: usize = Board::BOARD_COLS;
            Self {
                cols: if COLS % 2 == 0 {
                    Some((COLS / 2 - 1, COLS / 2))
                } else {
                    Some((COLS / 2, COLS / 2))
                },
            }
        }

        pub fn next_cols(&mut self) -> Option<(usize, usize)> {
            self.cols.map(|cols| {
                if cols.0 > 0 {
                    self.cols = Some((cols.0 - 1, cols.1 + 1));
                } else {
                    self.cols = None
                }
                cols
            })
        }
    }

    impl Default for LineClearPhase {
        fn default() -> Self {
            Self::new()
        }
    }

    pub(super) fn tick_system(
        time: Res<Time>,
        mut query: Query<(&mut Handle<Image>, &BoardBlockEntityMarker)>,
        mut e_play_sound: EventWriter<PlaySoundEvent>,
        mut player_data: ResMut<PlayerData>,
        mut player_state: ResMut<NextState<PlayerState>>,
        block_image_assets: Res<BlockImageAssets>,
    ) {
        player_data.game_timer.tick(time.delta());
        let threshold = player_data.line_clear_tick.threshold();
        if player_data.game_timer.commit(threshold) {
            if let Some((left, right)) = player_data.line_clear_phase.next_cols() {
                for (mut image, coordinate) in query.iter_mut() {
                    if (coordinate.0 == left || coordinate.0 == right)
                        && player_data.line_clear_rows.contains(&coordinate.1)
                    {
                        *image = block_image_assets.get_default();
                    }
                }
            } else {
                if player_data.board.clear_lines() {
                    e_play_sound.send(PlaySoundEvent::LevelUp);
                }
                player_data.fall_tick = FallTick::new(player_data.board.level(), false);
                player_state.set(PlayerState::GameUpdateBlockImageAssets);
            }
        }
    }
}

mod state_game_update_assets {
    use super::*;

    pub(super) fn update_block_image_assets(
        mut player_state: ResMut<NextState<PlayerState>>,
        mut image_assets: ResMut<Assets<Image>>,
        mut block_image_assets: ResMut<BlockImageAssets>,
        player_data: Res<PlayerData>,
    ) {
        *block_image_assets = BlockImageAssets {
            images: PieceShape::iter()
                .map(|shape| image_assets.add(get_block_image(*shape, player_data.board.level())))
                .collect(),
            default: image_assets.add(get_default_block_image()),
        };
        player_state.set(PlayerState::GameEntryDelay);
    }
}

mod state_game_entry_delay {
    use super::*;

    pub(super) fn tick_system(
        time: Res<Time>,
        mut query: ParamSet<(
            Query<(&mut Handle<Image>, &BoardBlockEntityMarker)>,
            Query<(&mut Transform, &mut Handle<Image>), With<CurrPieceEntityMarker>>,
            Query<(&mut Transform, &mut Handle<Image>), With<NextPieceEntityMarker>>,
        )>,
        mut player_data: ResMut<PlayerData>,
        mut player_state: ResMut<NextState<PlayerState>>,
        block_image_assets: Res<BlockImageAssets>,
    ) {
        player_data.game_timer.tick(time.delta());
        let threshold = player_data.entry_delay_tick.threshold();
        if player_data.game_timer.commit(threshold) {
            player_data.board.switch_to_next_piece();

            query.p0().iter_mut().for_each(|(mut image, coordinate)| {
                *image = block_image_assets.get_image_or_default(
                    player_data
                        .board
                        .block(coordinate.0 as i32, coordinate.1 as i32),
                );
            });

            std::iter::zip(
                query.p1().iter_mut(),
                player_data.board.get_curr_piece_blocks(),
            )
            .for_each(|((mut transform, mut image), blk)| {
                *image = block_image_assets.get_image(player_data.board.get_curr_piece().shape());
                transform.translation = player_data.rc.curr_piece_translation(blk.0, blk.1);
            });
            std::iter::zip(
                query.p2().iter_mut(),
                player_data.board.get_next_piece().to_blocks(),
            )
            .for_each(|((mut transform, mut image), blk)| {
                *image = block_image_assets.get_image(player_data.board.get_next_piece().shape());
                transform.translation = player_data.rc.next_piece_translation(blk.0, blk.1);
            });

            player_state.set(PlayerState::GameRunning);
        }
    }
}

mod state_game_pause {
    use super::*;

    pub(super) fn handle_input_system(
        keys: Res<ButtonInput<KeyCode>>,
        buttons: Res<ButtonInput<GamepadButton>>,
        controller: Res<Controller>,
        mut query: ParamSet<(
            Query<&mut Visibility, With<BoardCoverEntityMarker>>,
            Query<&mut Visibility, With<NextPieceSlotCoverEntityMarker>>,
        )>,
        mut player_state: ResMut<NextState<PlayerState>>,
    ) {
        let clicked = if let Some(gamepad) = controller.gamepad {
            buttons.just_pressed(GamepadButton {
                gamepad,
                button_type: GamepadButtonType::Start,
            })
        } else {
            false
        };

        if clicked || keys.just_pressed(KeyCode::Enter) {
            *query.p0().single_mut() = Visibility::Hidden;
            *query.p1().single_mut() = Visibility::Hidden;
            player_state.set(PlayerState::GameRunning);
        }
    }
}

mod state_game_over {
    use super::*;

    pub(super) fn handle_input_system(
        keys: Res<ButtonInput<KeyCode>>,
        buttons: Res<ButtonInput<GamepadButton>>,
        controller: Res<Controller>,
        mut app_state: ResMut<NextState<AppState>>,
    ) {
        let clicked = if let Some(gamepad) = controller.gamepad {
            buttons.just_pressed(GamepadButton {
                gamepad,
                button_type: GamepadButtonType::Start,
            })
        } else {
            false
        };

        if clicked || keys.just_pressed(KeyCode::Enter) {
            app_state.set(AppState::Menu);
        }
    }
}
