use bevy::{
    color::palettes::css::{BLACK, GREEN, RED, WHITE, YELLOW},
    prelude::*,
};

use crate::{app_state::AppState, utility::despawn_all};

use super::{
    board::Board,
    tick::{duration_to_ticks, DelayAutoShiftTick, FallTick, LockdownDelayTick, PressDownTick},
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
                    setup_screen,
                    running_tick_system,
                    handle_input_system,
                    curr_piece_fall_system,
                    update_statistic_system,
                )
                    .chain()
                    .run_if(in_state(PlayerState::Running)),
                lockdown_delay_tick_system.run_if(in_state(PlayerState::LockdownDelay)),
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
    Running,
    LockdownDelay,
}

#[derive(Resource)]
pub struct PlayerData {
    board: Board,
    fall_tick: FallTick,
    press_down_tick: PressDownTick,
    das_tick: DelayAutoShiftTick,
    lockdown_delay_tick: LockdownDelayTick,
    setup_screen: bool,
}

impl PlayerData {
    pub fn new(start_level: usize) -> Self {
        Self {
            board: Board::new(start_level),
            fall_tick: FallTick::default(),
            press_down_tick: PressDownTick::default(),
            das_tick: DelayAutoShiftTick::default(),
            lockdown_delay_tick: LockdownDelayTick::default(),
            setup_screen: true,
        }
    }
}

impl Default for PlayerData {
    fn default() -> Self {
        Self::new(0)
    }
}

fn setup_screen(
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

fn running_tick_system(time: Res<Time>, mut player_data: ResMut<PlayerData>) {
    player_data.fall_tick.tick(time.delta());
    player_data.press_down_tick.tick(time.delta());
    player_data.das_tick.tick(time.delta());
}

fn handle_input_system(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    q_curr: Query<Entity, With<CurrPieceEntityMarker>>,
    mut player_data: ResMut<PlayerData>,
) {
    let mut respawn = false;
    match (
        keys.just_pressed(KeyCode::KeyA),
        keys.just_pressed(KeyCode::KeyD),
    ) {
        (true, false) => respawn |= player_data.board.move_piece_left(),
        (false, true) => respawn |= player_data.board.move_piece_right(),
        _ => (),
    }
    match (keys.pressed(KeyCode::KeyA), keys.pressed(KeyCode::KeyD)) {
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
        (false, false) => player_data.das_tick.reset(),
        _ => (),
    }

    if keys.pressed(KeyCode::KeyS) {
        if player_data.press_down_tick.consume() {
            respawn |= player_data.board.move_piece_down();
            player_data.fall_tick.reset();
        }
    } else {
        player_data.press_down_tick.reset();
    }
    if keys.just_pressed(KeyCode::Comma) {
        respawn |= player_data.board.rotate_piece_counter_clockwise();
    }
    if keys.just_pressed(KeyCode::Period) {
        respawn |= player_data.board.rotate_piece_clockwise();
    }
    if respawn {
        q_curr
            .iter()
            .for_each(|entity| commands.entity(entity).despawn());
        spawn_curr_piece(commands.reborrow(), &player_data);
    }
}

fn curr_piece_fall_system(
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
        } else {
            let min_y = player_data
                .board
                .get_curr_piece_blocks()
                .iter()
                .fold(19, |acc, blk| acc.min(blk.1 as u64));
            player_data.lockdown_delay_tick.reset(min_y);
            player_state.set(PlayerState::LockdownDelay);
        }
    }
}

fn update_statistic_system(
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

fn lockdown_delay_tick_system(
    time: Res<Time>,
    mut player_data: ResMut<PlayerData>,
    mut player_state: ResMut<NextState<PlayerState>>,
) {
    player_data.lockdown_delay_tick.tick(time.delta());
    if player_data.lockdown_delay_tick.consume() {
        player_data.board.lock_and_switch();
        player_data.board.clear_lines();
        player_data.setup_screen = true;
        player_state.set(PlayerState::Running);
    }
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
