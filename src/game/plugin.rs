use bevy::{
    color::palettes::css::{BLACK, GREEN, RED, WHITE, YELLOW},
    prelude::*,
};

use crate::{app_state::AppState, controller::Controller, utility::despawn_all};

use super::{
    board::Board,
    piece::Block,
    tick::{duration_to_ticks, EntryDelayTick, FallTick, LineClearTick},
    timer::{DelayAutoShiftTimer, GameTimer, PressDownTimer},
};

pub fn setup(app: &mut App) {
    app.insert_resource(PlayerData::default())
        .init_state::<PlayerState>()
        .add_systems(OnEnter(AppState::Game), setup_screen)
        .add_systems(OnExit(AppState::Game), despawn_all::<GameEntityMarker>)
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
                state_game_entry_delay::tick_system.run_if(in_state(PlayerState::GameEntryDelay)),
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
const NEXT_PIECE_TRANSLATION: Vec3 = Vec3::new(WIDTH, 0.0, CURR_PIECE_LAYER);
const DAS_TRANSLATION: Vec3 = Vec3::new(-WIDTH, BLOCK_SIZE * 5.0, BOARD_LAYER);
const BURNED_TRANSLATION: Vec3 = Vec3::new(-WIDTH, BLOCK_SIZE * 2.0, BOARD_LAYER);
const TETRIS_COUNT_TRANSLATION: Vec3 = Vec3::new(-WIDTH, BLOCK_SIZE * 1.0, BOARD_LAYER);
const TETRIS_RATE_TRANSLATION: Vec3 = Vec3::new(-WIDTH, BLOCK_SIZE * 0.0, BOARD_LAYER);
const DROUGHT_RATE_TRANSLATION: Vec3 = Vec3::new(-WIDTH, -BLOCK_SIZE * 2.0, BOARD_LAYER);

#[derive(Component)]
struct GameEntityMarker;

#[derive(Component)]
struct BoardEntityMarker;

#[derive(Component)]
struct BoardBlockEntityMarker(usize, usize);

#[derive(Component)]
struct LinesEntityMarker;

#[derive(Component)]
struct ScoreEntityMarker;

#[derive(Component)]
struct LevelEntityMarker;

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

#[derive(Component)]
struct NextPieceEntityMarker;

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum PlayerState {
    #[default]
    GameRunning,
    GameLineClear,
    GameEntryDelay,
    GameOver,
}

#[derive(Resource)]
pub struct PlayerData {
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

fn setup_screen(mut commands: Commands, player_data: ResMut<PlayerData>) {
    spawn_board(commands.reborrow(), &player_data);
    spawn_statistic(commands.reborrow());
    spawn_curr_piece(commands.reborrow(), &player_data);
    spawn_next_piece(commands.reborrow(), &player_data);
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
        BoardEntityMarker,
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
                        .insert(GameEntityMarker)
                        .insert(BoardEntityMarker)
                        .insert(BoardBlockEntityMarker(x, y));
                } else {
                    commands
                        .spawn(new_block(
                            board_index_to_translation(x as i32, y as i32, BLOCK_LAYER),
                            BLACK.into(),
                        ))
                        .insert(GameEntityMarker)
                        .insert(BoardEntityMarker)
                        .insert(BoardBlockEntityMarker(x, y));
                }
            })
        });
}

fn spawn_statistic(mut commands: Commands) {
    commands.spawn((
        new_text("", LINES_TRANSLATION),
        GameEntityMarker,
        LinesEntityMarker,
    ));
    commands.spawn((
        new_text("", SCORE_TRANSLATION),
        GameEntityMarker,
        ScoreEntityMarker,
    ));
    commands.spawn((
        new_text("", LEVEL_TRANSLATION),
        GameEntityMarker,
        LevelEntityMarker,
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
            commands
                .spawn(new_block(
                    next_piece_index_to_translation(blk.0, blk.1),
                    RED.into(),
                ))
                .insert(GameEntityMarker)
                .insert(NextPieceEntityMarker);
        });
}

fn board_index_to_translation(x: i32, y: i32, z: f32) -> Vec3 {
    Vec3::new(
        (x as f32 + 0.5) * BLOCK_SIZE - WIDTH / 2.0,
        (y as f32 + 0.5) * BLOCK_SIZE - HEIGHT / 2.0,
        z,
    )
}

fn next_piece_index_to_translation(x: i32, y: i32) -> Vec3 {
    Vec3::new(x as f32 * BLOCK_SIZE, y as f32 * BLOCK_SIZE, 0.0) + NEXT_PIECE_TRANSLATION
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

fn move_board_blocks<'a>(
    iter: impl Iterator<Item = (usize, Mut<'a, Transform>)>,
    blocks: [Block; 4],
) {
    for (index, mut transform) in iter {
        let blk = blocks[index];
        transform.translation = board_index_to_translation(blk.0, blk.1, CURR_PIECE_LAYER);
    }
}

fn move_next_piece_blocks<'a>(
    iter: impl Iterator<Item = (usize, Mut<'a, Transform>)>,
    blocks: [Block; 4],
) {
    for (index, mut transform) in iter {
        let blk = blocks[index];
        transform.translation = next_piece_index_to_translation(blk.0, blk.1);
    }
}

fn update_statistic_system(
    mut set: ParamSet<(
        Query<&mut Text, With<LinesEntityMarker>>,
        Query<&mut Text, With<ScoreEntityMarker>>,
        Query<&mut Text, With<LevelEntityMarker>>,
        Query<&mut Text, With<DASEntityMarker>>,
        Query<&mut Text, With<BurnedEntityMarker>>,
        Query<&mut Text, With<TetrisCountEntityMarker>>,
        Query<&mut Text, With<TetrisRateEntityMarker>>,
        Query<&mut Text, With<DroughtEntityMarker>>,
    )>,
    player_data: ResMut<PlayerData>,
) {
    if let Ok(mut text) = set.p0().get_single_mut() {
        text.sections[0].value = format!("LINES {:04}", player_data.board.lines);
    }
    if let Ok(mut text) = set.p1().get_single_mut() {
        text.sections[0].value = format!("SCORE {:06}", player_data.board.score);
    }
    if let Ok(mut text) = set.p2().get_single_mut() {
        text.sections[0].value = format!("LEVEL {:02}", player_data.board.level());
    }
    if let Ok(mut text) = set.p3().get_single_mut() {
        let ticks = duration_to_ticks(player_data.das_timer.duration());
        text.sections[1].value = format!("{:02}", ticks);
        if ticks >= 10 {
            text.sections[1].style.color = GREEN.into();
        } else {
            text.sections[1].style.color = RED.into();
        }
    }
    if let Ok(mut text) = set.p4().get_single_mut() {
        text.sections[1].value = format!("{:4}", player_data.board.burned());
    }
    if let Ok(mut text) = set.p5().get_single_mut() {
        text.sections[1].value = format!("{:4}", player_data.board.tetris_count);
    }
    if let Ok(mut text) = set.p6().get_single_mut() {
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
    if let Ok(mut text) = set.p7().get_single_mut() {
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

mod state_game_running {
    use state_game_line_clear::LineClearPhase;

    use crate::game::board::BOARD_COLS;

    use super::*;

    pub(super) fn tick_system(time: Res<Time>, mut player_data: ResMut<PlayerData>) {
        player_data.game_timer.tick(time.delta());
        player_data.press_down_timer.tick(time.delta());
        player_data.das_timer.tick(time.delta());
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

    pub(super) fn handle_input_system(
        keys: Res<ButtonInput<KeyCode>>,
        buttons: Res<ButtonInput<GamepadButton>>,
        controller: Res<Controller>,
        mut q_curr: Query<&mut Transform, With<CurrPieceEntityMarker>>,
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
            move_board_blocks(
                q_curr.iter_mut().enumerate(),
                player_data.board.get_curr_piece_blocks(),
            );
        }
    }

    fn handle_input(inputs: GameRunningInputs, player_data: &mut PlayerData) -> bool {
        let mut moved = false;

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
            player_data.fall_tick = FallTick::new(player_data.board.level(), false);
            player_data.press_down_timer.reset();
        }

        if !inputs.down.1 {
            player_data.press_down_timer.reset();

            if inputs.left.0 || inputs.right.0 {
                player_data.das_timer.reset();
                match (inputs.left.0, inputs.right.0) {
                    (true, false) => moved |= player_data.board.move_piece_left(),
                    (false, true) => moved |= player_data.board.move_piece_right(),
                    _ => (),
                }
            } else {
                match (inputs.left.1, inputs.right.1) {
                    (true, false) => {
                        if !player_data.board.is_left_movable() {
                            player_data.das_timer.reset_max();
                        } else if player_data.das_timer.commit() {
                            moved |= player_data.board.move_piece_left();
                        }
                    }
                    (false, true) => {
                        if !player_data.board.is_right_movable() {
                            player_data.das_timer.reset_max();
                        } else if player_data.das_timer.commit() {
                            moved |= player_data.board.move_piece_right();
                        }
                    }
                    _ => (),
                }
            }
        }

        if inputs.rotate_clockwise {
            moved |= player_data.board.rotate_piece_clockwise();
        }
        if inputs.rotate_counter_clockwise {
            moved |= player_data.board.rotate_piece_counter_clockwise();
        }

        moved
    }

    pub(super) fn curr_piece_fall_system(
        mut q_board_block: Query<(&mut Sprite, &BoardBlockEntityMarker)>,
        mut q_curr: Query<&mut Transform, With<CurrPieceEntityMarker>>,
        mut player_data: ResMut<PlayerData>,
        mut player_state: ResMut<NextState<PlayerState>>,
    ) {
        let threshold = player_data.fall_tick.threshold();
        if player_data.game_timer.commit(threshold) {
            player_data.fall_tick = FallTick::new(player_data.board.level(), false);

            if player_data.board.move_piece_down() {
                move_board_blocks(
                    q_curr.iter_mut().enumerate(),
                    player_data.board.get_curr_piece_blocks(),
                );
            } else if !player_data.board.is_curr_position_valid() {
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
                q_curr.iter_mut().for_each(|mut transform| {
                    transform.translation.z = BOARD_LAYER - 1.0; // make invisible
                });
                for (mut sprite, index) in q_board_block.iter_mut() {
                    if player_data.board.blocks[index.1][index.0] {
                        sprite.color = WHITE.into();
                    } else {
                        sprite.color = BLACK.into();
                    }
                }

                let lines = player_data.board.get_line_clear_indexes();
                if lines.len() > 0 {
                    player_data.line_clear_tick = LineClearTick::new((BOARD_COLS + 1) / 2);
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
    use crate::game::board::BOARD_COLS;

    use super::*;

    pub(super) struct LineClearPhase {
        cols: Option<(usize, usize)>, // (left, right)
    }

    impl LineClearPhase {
        pub fn new() -> Self {
            Self {
                cols: if BOARD_COLS % 2 == 0 {
                    Some((BOARD_COLS / 2 - 1, BOARD_COLS / 2))
                } else {
                    Some((BOARD_COLS / 2, BOARD_COLS / 2))
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
        mut q_board_block: Query<(&mut Sprite, &BoardBlockEntityMarker)>,
        mut player_data: ResMut<PlayerData>,
        mut player_state: ResMut<NextState<PlayerState>>,
    ) {
        player_data.game_timer.tick(time.delta());
        let threshold = player_data.line_clear_tick.threshold();
        if player_data.game_timer.commit(threshold) {
            if let Some((left, right)) = player_data.line_clear_phase.next_cols() {
                for (mut sprite, index) in q_board_block.iter_mut() {
                    if (index.0 == left || index.0 == right)
                        && player_data.line_clear_rows.contains(&index.1)
                    {
                        sprite.color = BLACK.into();
                    }
                }
            } else {
                player_data.board.clear_lines();
                player_data.fall_tick = FallTick::new(player_data.board.level(), false);
                player_state.set(PlayerState::GameEntryDelay);
            }
        }
    }
}

mod state_game_entry_delay {
    use super::*;

    pub(super) fn tick_system(
        time: Res<Time>,
        mut q_board_block: Query<(&mut Sprite, &BoardBlockEntityMarker)>,
        mut q_piece: ParamSet<(
            Query<&mut Transform, With<CurrPieceEntityMarker>>,
            Query<&mut Transform, With<NextPieceEntityMarker>>,
        )>,
        mut player_data: ResMut<PlayerData>,
        mut player_state: ResMut<NextState<PlayerState>>,
    ) {
        player_data.game_timer.tick(time.delta());
        let threshold = player_data.entry_delay_tick.threshold();
        if player_data.game_timer.commit(threshold) {
            player_data.board.switch_to_next_piece();

            for (mut sprite, index) in q_board_block.iter_mut() {
                if player_data.board.blocks[index.1][index.0] {
                    sprite.color = WHITE.into();
                } else {
                    sprite.color = BLACK.into();
                }
            }

            move_board_blocks(
                q_piece.p0().iter_mut().enumerate(),
                player_data.board.get_curr_piece_blocks(),
            );
            move_next_piece_blocks(
                q_piece.p1().iter_mut().enumerate(),
                player_data.board.get_next_piece_blocks(),
            );

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
