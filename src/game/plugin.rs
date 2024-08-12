use bevy::prelude::*;

use crate::{app_state::AppState, utility::despawn_all};

use super::board::Board;

pub fn setup(app: &mut App) {
    app.insert_resource(PlayerData::default())
        .add_systems(OnEnter(AppState::Game), setup_screen)
        .add_systems(OnExit(AppState::Game), despawn_all::<GameEntityMarker>)
        .add_systems(
            Update,
            (curr_piece_fall_down_system, switch_to_next_piece_system)
                .run_if(in_state(AppState::Game)),
        );
}

const BLOCK_SIZE: f32 = 40.0;
const BLOCK_PADDING: f32 = BLOCK_SIZE / 20.0;
const WIDTH: f32 = BLOCK_SIZE * 10.0;
const HEIGHT: f32 = BLOCK_SIZE * 20.0;

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
        Self {
            board: Board::default(),
            falldown_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            switch_to_next_piece: false,
        }
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
                translation: Vec3::ZERO,
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
                        .spawn(new_block(x as i32, y as i32, 1.0, Color::WHITE))
                        .insert(GameEntityMarker);
                } else {
                    commands
                        .spawn(new_block(
                            x as i32,
                            y as i32,
                            1.0,
                            Color::srgb(0.1, 0.1, 0.1),
                        ))
                        .insert(GameEntityMarker);
                }
            })
        });
}

fn spawn_curr_piece(mut commands: Commands, player_data: &PlayerData) {
    player_data
        .board
        .get_curr_piece_blocks()
        .iter()
        .for_each(|blk| {
            commands
                .spawn(new_block(blk.0, blk.1, 2.0, Color::srgb(1.0, 0.0, 0.0)))
                .insert(GameEntityMarker)
                .insert(CurrPieceEntityMarker);
        });
}

fn curr_piece_fall_down_system(
    mut q_curr: Query<&mut Transform, With<CurrPieceEntityMarker>>,
    time: Res<Time>,
    mut player_data: ResMut<PlayerData>,
) {
    if player_data.falldown_timer.tick(time.delta()).finished() {
        if player_data.board.move_curr_piece_down() {
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
