use bevy::prelude::*;

use crate::{app_state::AppState, utility::despawn_all};

use super::{board::Board, consts::*, level};

pub fn setup(app: &mut App) {
    app.insert_resource(PlayerData::default())
        .add_systems(OnEnter(AppState::Game), setup_screen)
        .add_systems(OnExit(AppState::Game), despawn_all::<GameEntityMarker>)
        .add_systems(
            Update,
            (
                tick_system,
                handle_input_system,
                curr_piece_fall_system,
                lock_and_switch_system,
                setup_screen,
            )
                .chain()
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
const DAS_TRANSLATION: Vec3 = Vec3::new(-WIDTH, HEIGHT / 3.0, BOARD_LAYER);

#[derive(Component)]
struct GameEntityMarker;

#[derive(Component)]
struct DASEntityMarker;

#[derive(Component)]
struct CurrPieceEntityMarker;

#[derive(Resource)]
struct PlayerData {
    board: Board,
    tick_timer: Timer,
    fall_timer: Timer,
    pressed_down_tick: i32,
    das_tick: i32,
    lock_and_switch: bool,
    setup_screen: bool,
}

impl PlayerData {
    fn new() -> Self {
        Self {
            board: Board::new(0),
            tick_timer: Timer::from_seconds(1.0 / 60.0, TimerMode::Repeating),
            fall_timer: Timer::from_seconds(1.6, TimerMode::Once),
            pressed_down_tick: TICK_MOVE_DOWN_INPUT_RELEASED_RESET,
            das_tick: TICK_DAS_RELEASED_RESET,
            lock_and_switch: false,
            setup_screen: true,
        }
    }

    fn reset_timer(&mut self) {
        self.fall_timer =
            Timer::from_seconds(level::drop_time(self.board.level()), TimerMode::Once);
    }
}

impl Default for PlayerData {
    fn default() -> Self {
        Self::new()
    }
}

fn setup_screen(
    mut commands: Commands,
    mut player_data: ResMut<PlayerData>,
    q_das: Query<Entity, With<DASEntityMarker>>,
) {
    if std::mem::replace(&mut player_data.setup_screen, false) {
        spawn_board(commands.reborrow(), &player_data);
        spawn_curr_piece(commands.reborrow(), &player_data);
        spawn_next_piece(commands.reborrow(), &player_data);
    }

    if let Ok(entity) = q_das.get_single() {
        commands.entity(entity).despawn();
    }
    spawn_das_indicator(commands.reborrow(), &player_data);
}

fn spawn_board(mut commands: Commands, player_data: &PlayerData) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, BOARD_LAYER),
                ..default()
            },
            sprite: Sprite {
                color: Color::BLACK,
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
                            Color::WHITE,
                        ))
                        .insert(GameEntityMarker);
                } else {
                    commands
                        .spawn(new_block(
                            board_index_to_translation(x as i32, y as i32, BLOCK_LAYER),
                            Color::srgb(0.1, 0.1, 0.1),
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
}

fn spawn_das_indicator(mut commands: Commands, player_data: &PlayerData) {
    commands.spawn((
        new_text(format!("DAS {:02}", player_data.das_tick), DAS_TRANSLATION),
        GameEntityMarker,
        DASEntityMarker,
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
                    Color::srgb(1.0, 0.0, 0.0),
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
                .spawn(new_block(translation, Color::srgb(1.0, 0.0, 0.0)))
                .insert(GameEntityMarker);
        });
}

fn tick_system(time: Res<Time>, mut player_data: ResMut<PlayerData>) {
    if player_data.tick_timer.tick(time.delta()).just_finished() {
        player_data.pressed_down_tick += 1;
        player_data.das_tick = std::cmp::min(16, player_data.das_tick + 1);
    }
}

fn handle_input_system(
    mut commands: Commands,
    q_keys: Res<ButtonInput<KeyCode>>,
    q_curr: Query<Entity, With<CurrPieceEntityMarker>>,
    mut player_data: ResMut<PlayerData>,
) {
    let mut respawn = false;
    if q_keys.just_pressed(KeyCode::KeyA) {
        respawn |= player_data.board.move_piece_left();
    }
    if q_keys.just_pressed(KeyCode::KeyD) {
        respawn |= player_data.board.move_piece_right();
    }
    if q_keys.pressed(KeyCode::KeyA) {
        if !player_data.board.is_left_movable() {
            player_data.das_tick = TICK_DAS_PRESSED_TRIGGER;
        } else if player_data.das_tick >= TICK_DAS_PRESSED_TRIGGER {
            respawn |= player_data.board.move_piece_left();
            player_data.das_tick = TICK_DAS_PRESSED_RESET;
        }
    }
    if q_keys.pressed(KeyCode::KeyD) {
        if !player_data.board.is_right_movable() {
            player_data.das_tick = TICK_DAS_PRESSED_TRIGGER;
        } else if player_data.das_tick >= TICK_DAS_PRESSED_TRIGGER {
            respawn |= player_data.board.move_piece_right();
            player_data.das_tick = TICK_DAS_PRESSED_RESET;
        }
    }
    if !q_keys.pressed(KeyCode::KeyA) && !q_keys.pressed(KeyCode::KeyD) {
        player_data.das_tick = TICK_DAS_RELEASED_RESET;
    }
    if q_keys.pressed(KeyCode::KeyS) {
        if player_data.pressed_down_tick >= TICK_MOVE_DOWN_INPUT_PRESSED_TRIGGER {
            respawn |= player_data.board.move_piece_down();
            player_data.reset_timer();
            player_data.pressed_down_tick = TICK_MOVE_DOWN_INPUT_PRESSED_RESET;
        }
    } else {
        player_data.pressed_down_tick = TICK_MOVE_DOWN_INPUT_RELEASED_RESET;
    }
    if q_keys.just_pressed(KeyCode::Comma) {
        respawn |= player_data.board.rotate_piece_counter_clockwise();
    }
    if q_keys.just_pressed(KeyCode::Period) {
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
    time: Res<Time>,
    mut player_data: ResMut<PlayerData>,
) {
    if player_data.fall_timer.tick(time.delta()).finished() {
        if player_data.board.move_piece_down() {
            for mut blk in q_curr.iter_mut() {
                blk.translation.y -= BLOCK_SIZE;
            }
            player_data.reset_timer();
        } else {
            player_data.lock_and_switch = true;
        }
    }
}

fn lock_and_switch_system(
    mut commands: Commands,
    q_game: Query<Entity, With<GameEntityMarker>>,
    mut player_data: ResMut<PlayerData>,
) {
    if std::mem::replace(&mut player_data.lock_and_switch, false) {
        q_game
            .iter()
            .for_each(|entity| commands.entity(entity).despawn());
        player_data.board.lock_and_switch();
        player_data.board.clear_lines();
        player_data.reset_timer();

        player_data.setup_screen = true;
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

fn new_text(text: String, translation: Vec3) -> Text2dBundle {
    Text2dBundle {
        text: Text::from_section(
            text,
            TextStyle {
                font_size: BLOCK_SIZE,
                color: Color::WHITE,
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
