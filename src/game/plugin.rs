use bevy::prelude::*;

use crate::{app_state::AppState, utility::despawn_all};

use super::{board::Board, level};

pub fn setup(app: &mut App) {
    app.insert_resource(PlayerData::default())
        .add_systems(OnEnter(AppState::Game), setup_screen)
        .add_systems(OnExit(AppState::Game), despawn_all::<GameEntityMarker>)
        .add_systems(
            Update,
            (
                handle_input_system,
                curr_piece_fall_down_system,
                switch_to_next_piece_system,
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

#[derive(Component)]
struct GameEntityMarker;

#[derive(Component)]
struct CurrPieceEntityMarker;

#[derive(Resource)]
struct PlayerData {
    board: Board,
    falldown_timer: Timer,
    switch_to_next_piece: bool,
}

impl PlayerData {
    fn new() -> Self {
        let mut me = Self {
            board: Board::new(10),
            falldown_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            switch_to_next_piece: false,
        };
        me.update_timer();
        me
    }

    fn update_timer(&mut self) {
        self.falldown_timer =
            Timer::from_seconds(level::drop_time(self.board.level()), TimerMode::Repeating);
    }
}

impl Default for PlayerData {
    fn default() -> Self {
        Self::new()
    }
}

fn setup_screen(mut commands: Commands, player_data: Res<PlayerData>) {
    spawn_board(commands.reborrow(), &player_data);
    spawn_curr_piece(commands.reborrow(), &player_data)
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
                        .spawn(new_block(x as i32, y as i32, BLOCK_LAYER, Color::WHITE))
                        .insert(GameEntityMarker);
                } else {
                    commands
                        .spawn(new_block(
                            x as i32,
                            y as i32,
                            BLOCK_LAYER,
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

fn spawn_curr_piece(mut commands: Commands, player_data: &PlayerData) {
    player_data
        .board
        .get_curr_piece_blocks()
        .iter()
        .for_each(|blk| {
            commands
                .spawn(new_block(
                    blk.0,
                    blk.1,
                    CURR_PIECE_LAYER,
                    Color::srgb(1.0, 0.0, 0.0),
                ))
                .insert(GameEntityMarker)
                .insert(CurrPieceEntityMarker);
        });
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
    if q_keys.pressed(KeyCode::KeyS) {
        respawn |= player_data.board.move_piece_down();
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

fn curr_piece_fall_down_system(
    mut q_curr: Query<&mut Transform, With<CurrPieceEntityMarker>>,
    time: Res<Time>,
    mut player_data: ResMut<PlayerData>,
) {
    if player_data.falldown_timer.tick(time.delta()).finished() {
        if player_data.board.move_piece_down() {
            for mut blk in q_curr.iter_mut() {
                blk.translation.y -= BLOCK_SIZE;
            }
        } else {
            for blk in player_data.board.get_curr_piece_blocks() {
                player_data.board.blocks[blk.1 as usize][blk.0 as usize] = true;
            }

            player_data.switch_to_next_piece = true;
        }
    }
}

fn switch_to_next_piece_system(
    mut commands: Commands,
    q_game: Query<Entity, With<GameEntityMarker>>,
    mut player_data: ResMut<PlayerData>,
) {
    if std::mem::replace(&mut player_data.switch_to_next_piece, false) {
        q_game
            .iter()
            .for_each(|entity| commands.entity(entity).despawn());
        player_data.board.switch_to_next_piece();
        player_data.board.clear_lines();
        player_data.update_timer();

        spawn_board(commands.reborrow(), &player_data);
        spawn_curr_piece(commands.reborrow(), &player_data);
    }
}

fn index_to_translation(x: i32, y: i32) -> (f32, f32) {
    (
        (x as f32 + 0.5) * BLOCK_SIZE - WIDTH / 2.0,
        (y as f32 + 0.5) * BLOCK_SIZE - HEIGHT / 2.0,
    )
}

fn new_block(x: i32, y: i32, z: f32, color: Color) -> SpriteBundle {
    let (x, y) = index_to_translation(x, y);
    SpriteBundle {
        transform: Transform {
            translation: Vec3::new(x, y, z),
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
