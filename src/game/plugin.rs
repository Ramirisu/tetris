use bevy::{
    color::palettes::css::{BLACK, GREEN, RED, WHITE, YELLOW},
    prelude::*,
};

use crate::{app_state::AppState, controller::Controller, utility::despawn_all};

use super::{
    board::Board,
    tick::{duration_to_ticks, DelayAutoShiftTick, FallTick, LockDelayTick, PressDownTick},
};

pub fn setup(app: &mut App) {
    app.insert_resource(PlayerData::default())
        .init_state::<PlayerState>()
        .add_systems(OnExit(AppState::Game), despawn_all::<GameEntityMarker>)
        .add_systems(
            Update,
            (
                (
                    state_game_running::setup_screen,
                    state_game_running::tick_system,
                    state_game_running::handle_input_system,
                    state_game_running::curr_piece_fall_system,
                    state_game_running::update_statistic_system,
                )
                    .chain()
                    .run_if(in_state(PlayerState::GameRunning)),
                state_game_lock_delay::tick_system.run_if(in_state(PlayerState::GameLockDelay)),
                state_game_over::handle_input_system.run_if(in_state(PlayerState::GameOver)),
            )
                .run_if(in_state(AppState::Game)),
        );
}

const BOARD_LAYER: f32 = 0.0;
const BLOCK_LAYER: f32 = 1.0;
const CURR_PIECE_LAYER: f32 = 2.0;

const BLOCK_SIZE: f32 = 40.0;
const BLOCK_PADDING: f32 = BLOCK_SIZE / 20.0;
const WIDTH: f32 = BLOCK_SIZE * 10.0;
const HEIGHT: f32 = BLOCK_SIZE * 20.0;

const LINES_TRANSLATION: Vec3 = Vec3::new(0.0, HEIGHT / 2.0 + BLOCK_SIZE, BOARD_LAYER);
const SCORE_TRANSLATION: Vec3 = Vec3::new(WIDTH, HEIGHT / 3.0, BOARD_LAYER);
const LEVEL_TRANSLATION: Vec3 = Vec3::new(WIDTH, -HEIGHT / 3.0, BOARD_LAYER);
const NEXT_PIECE_TRANSLATION: Vec3 = Vec3::new(WIDTH, 0.0, 0.0);
const DAS_TRANSLATION: Vec3 = Vec3::new(-WIDTH, BLOCK_SIZE * 5.0, BOARD_LAYER);
const BURNED_TRANSLATION: Vec3 = Vec3::new(-WIDTH, BLOCK_SIZE * 2.0, BOARD_LAYER);
const TETRIS_COUNT_TRANSLATION: Vec3 = Vec3::new(-WIDTH, BLOCK_SIZE * 1.0, BOARD_LAYER);
const TETRIS_RATE_TRANSLATION: Vec3 = Vec3::new(-WIDTH, BLOCK_SIZE * 0.0, BOARD_LAYER);
const DROUGHT_RATE_TRANSLATION: Vec3 = Vec3::new(-WIDTH, -BLOCK_SIZE * 2.0, BOARD_LAYER);

#[derive(Component)]
struct GameEntityMarker;

#[derive(Component)]
struct DASEntityMarker;

#[derive(Component)]
struct BurnedEntityMarker;

#[derive(Component)]
struct TetrisCountEntityMarker;

#[derive(Component)]
struct TetrisRateEntityMarker;

#[derive(Component)]
struct DroughtEntityMarker;

#[derive(Component)]
struct CurrPieceEntityMarker;

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum PlayerState {
    #[default]
    GameRunning,
    GameLockDelay,
    GameOver,
}

#[derive(Resource)]
pub struct PlayerData {
    board: Board,
    fall_tick: FallTick,
    can_press_down: bool,
    press_down_tick: PressDownTick,
    das_tick: DelayAutoShiftTick,
    lock_delay_tick: LockDelayTick,
    setup_screen: bool,
}

impl PlayerData {
    pub fn new(start_level: usize) -> Self {
        Self {
            board: Board::new(start_level),
            fall_tick: FallTick::default(),
            can_press_down: false,
            press_down_tick: PressDownTick::default(),
            das_tick: DelayAutoShiftTick::default(),
            lock_delay_tick: LockDelayTick::default(),
            setup_screen: true,
        }
    }
}

impl Default for PlayerData {
    fn default() -> Self {
        Self::new(0)
    }
}

mod state_game_running {
    use super::*;

    pub(crate) fn setup_screen(
        mut commands: Commands,
        mut player_data: ResMut<PlayerData>,
        q_game: Query<Entity, With<GameEntityMarker>>,
    ) {
        if std::mem::replace(&mut player_data.setup_screen, false) {
            q_game
                .iter()
                .for_each(|entity| commands.entity(entity).despawn_recursive());
            spawn_board(commands.reborrow(), &player_data);
            spawn_curr_piece(commands.reborrow(), &player_data);
            spawn_next_piece(commands.reborrow(), &player_data);
        }
    }

    fn spawn_board(mut commands: Commands, player_data: &PlayerData) {
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, BOARD_LAYER),
                    ..default()
                },
                sprite: Sprite {
                    color: BLACK.into(),
                    custom_size: Some(Vec2::new(WIDTH, HEIGHT)),
                    ..default()
                },
                ..default()
            },
            GameEntityMarker,
        ));

        player_data
            .board
            .blocks
            .iter()
            .enumerate()
            .for_each(|(y, blks)| {
                blks.iter().enumerate().for_each(|(x, blk)| {
                    if *blk {
                        commands
                            .spawn(new_block(
                                board_index_to_translation(x as i32, y as i32, BLOCK_LAYER),
                                WHITE.into(),
                            ))
                            .insert(GameEntityMarker);
                    } else {
                        commands
                            .spawn(new_block(
                                board_index_to_translation(x as i32, y as i32, BLOCK_LAYER),
                                BLACK.into(),
                            ))
                            .insert(GameEntityMarker);
                    }
                })
            });

        commands.spawn((
            new_text(
                format!("LINES {:04}", player_data.board.lines),
                LINES_TRANSLATION,
            ),
            GameEntityMarker,
        ));
        commands.spawn((
            new_text(
                format!("SCORE {:06}", player_data.board.score),
                SCORE_TRANSLATION,
            ),
            GameEntityMarker,
        ));
        commands.spawn((
            new_text(
                format!("LEVEL {:02}", player_data.board.level()),
                LEVEL_TRANSLATION,
            ),
            GameEntityMarker,
        ));
        commands.spawn((
            new_texts(vec!["DAS ".into(), "".into()], DAS_TRANSLATION),
            GameEntityMarker,
            DASEntityMarker,
        ));
        commands.spawn((
            new_texts(vec!["BRN ".into(), "".into()], BURNED_TRANSLATION),
            GameEntityMarker,
            BurnedEntityMarker,
        ));
        commands.spawn((
            new_texts(vec!["TRT ".into(), "".into()], TETRIS_COUNT_TRANSLATION),
            GameEntityMarker,
            TetrisCountEntityMarker,
        ));
        commands.spawn((
            new_texts(vec!["TRT ".into(), "".into()], TETRIS_RATE_TRANSLATION),
            GameEntityMarker,
            TetrisRateEntityMarker,
        ));
        commands.spawn((
            new_texts(vec!["DRT ".into(), "".into()], DROUGHT_RATE_TRANSLATION),
            GameEntityMarker,
            DroughtEntityMarker,
        ));
    }

    fn spawn_curr_piece(mut commands: Commands, player_data: &PlayerData) {
        player_data
            .board
            .get_curr_piece_blocks()
            .iter()
            .for_each(|blk| {
                commands
                    .spawn(new_block(
                        board_index_to_translation(blk.0, blk.1, CURR_PIECE_LAYER),
                        RED.into(),
                    ))
                    .insert(GameEntityMarker)
                    .insert(CurrPieceEntityMarker);
            });
    }

    fn spawn_next_piece(mut commands: Commands, player_data: &PlayerData) {
        player_data
            .board
            .get_next_piece_blocks()
            .iter()
            .for_each(|blk| {
                let translation = Vec3::new(
                    blk.0 as f32 * BLOCK_SIZE,
                    blk.1 as f32 * BLOCK_SIZE,
                    CURR_PIECE_LAYER,
                ) + NEXT_PIECE_TRANSLATION;
                commands
                    .spawn(new_block(translation, RED.into()))
                    .insert(GameEntityMarker);
            });
    }

    fn board_index_to_translation(x: i32, y: i32, z: f32) -> Vec3 {
        Vec3::new(
            (x as f32 + 0.5) * BLOCK_SIZE - WIDTH / 2.0,
            (y as f32 + 0.5) * BLOCK_SIZE - HEIGHT / 2.0,
            z,
        )
    }

    fn new_block(translation: Vec3, color: Color) -> SpriteBundle {
        SpriteBundle {
            transform: Transform {
                translation,
                ..default()
            },
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(
                    BLOCK_SIZE - BLOCK_PADDING,
                    BLOCK_SIZE - BLOCK_PADDING,
                )),
                ..default()
            },
            ..default()
        }
    }

    fn new_text(text: impl Into<String>, translation: Vec3) -> Text2dBundle {
        Text2dBundle {
            text: Text::from_section(
                text,
                TextStyle {
                    font_size: BLOCK_SIZE,
                    color: WHITE.into(),
                    ..default()
                },
            ),
            transform: Transform {
                translation,
                ..default()
            },
            ..default()
        }
    }

    fn new_texts(texts: impl IntoIterator<Item = String>, translation: Vec3) -> Text2dBundle {
        Text2dBundle {
            text: Text::from_sections(texts.into_iter().map(|text| TextSection {
                value: text,
                style: TextStyle {
                    font_size: BLOCK_SIZE,
                    color: WHITE.into(),
                    ..default()
                },
            })),
            transform: Transform {
                translation,
                ..default()
            },
            ..default()
        }
    }

    pub(crate) fn tick_system(time: Res<Time>, mut player_data: ResMut<PlayerData>) {
        player_data.fall_tick.tick(time.delta());
        player_data.press_down_tick.tick(time.delta());
        player_data.das_tick.tick(time.delta());
    }

    pub struct GameRunningInputs {
        left: (bool, bool),  // (just_pressed, pressed)
        right: (bool, bool), // (just_pressed, pressed)
        down: (bool, bool),  // (just_pressed, pressed)
        rotate_clockwise: bool,
        rotate_counter_clockwise: bool,
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
        }
    }

    pub(crate) fn handle_input_system(
        mut commands: Commands,
        keys: Res<ButtonInput<KeyCode>>,
        buttons: Res<ButtonInput<GamepadButton>>,
        controller: Res<Controller>,
        q_curr: Query<Entity, With<CurrPieceEntityMarker>>,
        mut player_data: ResMut<PlayerData>,
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
            };
        }

        if handle_input(inputs, &mut player_data) {
            q_curr
                .iter()
                .for_each(|entity| commands.entity(entity).despawn());
            spawn_curr_piece(commands.reborrow(), &player_data);
        }
    }

    fn handle_input(inputs: GameRunningInputs, player_data: &mut PlayerData) -> bool {
        let mut respawn = false;

        if player_data.can_press_down {
            if inputs.down.1 {
                if player_data.press_down_tick.consume() {
                    respawn |= player_data.board.move_piece_down();
                }
            } else {
                player_data.can_press_down = false;
            }
        } else if inputs.down.0 {
            player_data.can_press_down = true;
            player_data.press_down_tick.reset();
        }

        if !inputs.down.1 {
            player_data.press_down_tick.reset();

            if inputs.left.0 || inputs.right.0 {
                player_data.das_tick.reset();
                match (inputs.left.0, inputs.right.0) {
                    (true, false) => respawn |= player_data.board.move_piece_left(),
                    (false, true) => respawn |= player_data.board.move_piece_right(),
                    _ => (),
                }
            } else {
                match (inputs.left.1, inputs.right.1) {
                    (true, false) => {
                        if !player_data.board.is_left_movable() {
                            player_data.das_tick.reset_max();
                        } else if player_data.das_tick.consume() {
                            respawn |= player_data.board.move_piece_left();
                        }
                    }
                    (false, true) => {
                        if !player_data.board.is_right_movable() {
                            player_data.das_tick.reset_max();
                        } else if player_data.das_tick.consume() {
                            respawn |= player_data.board.move_piece_right();
                        }
                    }
                    _ => (),
                }
            }
        }

        if inputs.rotate_clockwise {
            respawn |= player_data.board.rotate_piece_clockwise();
        }
        if inputs.rotate_counter_clockwise {
            respawn |= player_data.board.rotate_piece_counter_clockwise();
        }

        respawn
    }

    pub(crate) fn curr_piece_fall_system(
        mut q_curr: Query<&mut Transform, With<CurrPieceEntityMarker>>,
        mut player_data: ResMut<PlayerData>,
        mut player_state: ResMut<NextState<PlayerState>>,
    ) {
        let level = player_data.board.level();
        if player_data.fall_tick.consume(level) {
            if player_data.board.move_piece_down() {
                for mut blk in q_curr.iter_mut() {
                    blk.translation.y -= BLOCK_SIZE;
                }
            } else if !player_data.board.is_curr_position_valid() {
                player_state.set(PlayerState::GameOver);
            } else {
                let min_y = player_data
                    .board
                    .get_curr_piece_blocks()
                    .iter()
                    .fold(19, |acc, blk| acc.min(blk.1 as u64));
                player_data.lock_delay_tick.reset(min_y);
                player_data.can_press_down = false; // keep pressing down will not affect next piece
                player_state.set(PlayerState::GameLockDelay);
            }
        }
    }

    pub(crate) fn update_statistic_system(
        mut set: ParamSet<(
            Query<&mut Text, With<DASEntityMarker>>,
            Query<&mut Text, With<BurnedEntityMarker>>,
            Query<&mut Text, With<TetrisCountEntityMarker>>,
            Query<&mut Text, With<TetrisRateEntityMarker>>,
            Query<&mut Text, With<DroughtEntityMarker>>,
        )>,
        player_data: ResMut<PlayerData>,
    ) {
        if let Ok(mut text) = set.p0().get_single_mut() {
            let ticks = duration_to_ticks(player_data.das_tick.duration());
            text.sections[1].value = format!("{:02}", ticks);
            if ticks >= 10 {
                text.sections[1].style.color = GREEN.into();
            } else {
                text.sections[1].style.color = RED.into();
            }
        }
        if let Ok(mut text) = set.p1().get_single_mut() {
            text.sections[1].value = format!("{:4}", player_data.board.burned());
        }
        if let Ok(mut text) = set.p2().get_single_mut() {
            text.sections[1].value = format!("{:4}", player_data.board.tetris_count);
        }
        if let Ok(mut text) = set.p3().get_single_mut() {
            let rate = (player_data.board.tetris_rate() * 100.0).round() as usize;
            text.sections[1].value = format!("{:3}%", rate);
            if rate >= 80 {
                text.sections[1].style.color = GREEN.into();
            } else if rate >= 50 {
                text.sections[1].style.color = YELLOW.into();
            } else {
                text.sections[1].style.color = RED.into();
            }
        }
        if let Ok(mut text) = set.p4().get_single_mut() {
            text.sections[1].value = format!("{:4}", player_data.board.drought);
            if player_data.board.drought >= 14 {
                text.sections[1].style.color = RED.into();
            } else if player_data.board.drought >= 7 {
                text.sections[1].style.color = YELLOW.into();
            } else {
                text.sections[1].style.color = GREEN.into();
            }
        }
    }
}

mod state_game_lock_delay {
    use super::*;

    pub(crate) fn tick_system(
        time: Res<Time>,
        mut player_data: ResMut<PlayerData>,
        mut player_state: ResMut<NextState<PlayerState>>,
    ) {
        player_data.lock_delay_tick.tick(time.delta());
        if player_data.lock_delay_tick.consume() {
            player_data.board.lock_and_switch();
            player_data.board.clear_lines();
            player_data.setup_screen = true;
            player_state.set(PlayerState::GameRunning);
        }
    }
}

mod state_game_over {
    use super::*;

    pub(crate) fn handle_input_system(
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
