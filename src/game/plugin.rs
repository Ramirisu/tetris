use std::collections::HashMap;

use bevy::{
    color::palettes::css::{BLACK, GREEN, RED, WHITE, YELLOW},
    prelude::*,
};

use crate::{
    app_state::AppState, audio::plugin::PlaySoundEvent, inputs::PlayerInputs, utility::despawn_all,
};

use super::{
    asset::SquareImageAssets,
    board::Board,
    palette::{get_empty_square_image, get_square_image, SquareImageSize},
    piece::{Piece, PieceShape},
    player::{LineClearPhase, PlayerData, PlayerState},
    tick::{duration_to_ticks, EntryDelayTick, LineClearTick},
};

pub fn setup(app: &mut App) {
    app.insert_resource(PlayerData::default())
        .init_state::<PlayerState>()
        .add_systems(
            OnEnter(AppState::Game),
            (load_square_image_assets, setup_screen).chain(),
        )
        .add_systems(
            OnExit(AppState::Game),
            (despawn_all::<GameEntityMarker>, unload_square_image_assets),
        )
        .add_systems(
            Update,
            (
                (
                    state_game_running::tick_system,
                    state_game_running::handle_input_system,
                    state_game_running::curr_piece_fall_system,
                    update_statistics_system,
                )
                    .chain()
                    .run_if(in_state(PlayerState::GameRunning)),
                (state_game_line_clear::tick_system, update_statistics_system)
                    .chain()
                    .run_if(in_state(PlayerState::GameLineClear)),
                state_game_update_assets::update_square_image_assets
                    .run_if(in_state(PlayerState::GameUpdateSquareImageAssets)),
                state_game_entry_delay::tick_system.run_if(in_state(PlayerState::GameEntryDelay)),
                state_game_pause::handle_input_system.run_if(in_state(PlayerState::GamePause)),
                state_game_over::handle_input_system.run_if(in_state(PlayerState::GameOver)),
            )
                .run_if(in_state(AppState::Game)),
        );
}

#[derive(Component)]
struct GameEntityMarker;

#[derive(Component, Clone, Copy)]
struct BoardSquareEntityMarker(usize, usize);

impl Into<(usize, usize)> for &BoardSquareEntityMarker {
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
struct StatisticsEntityMarker;

#[derive(Component)]
struct PieceCountEntityMarker(PieceShape);

#[derive(Component)]
struct PieceCountCounterEntityMarker(PieceShape);

#[derive(Component)]
struct CurrPieceEntityMarker;

#[derive(Component)]
struct NextPieceEntityMarker;

fn load_square_image_assets(
    mut commands: Commands,
    mut image_assets: ResMut<Assets<Image>>,
    player_data: Res<PlayerData>,
) {
    commands.insert_resource(load_square_image_assets_impl(
        player_data.board.level(),
        &mut image_assets,
    ));
}

fn load_square_image_assets_impl(
    level: usize,
    image_assets: &mut Assets<Image>,
) -> SquareImageAssets {
    let mut images = HashMap::<SquareImageSize, Vec<Handle<Image>>>::new();
    for size in SquareImageSize::iter() {
        images.insert(
            *size,
            PieceShape::iter()
                .map(|shape| image_assets.add(get_square_image(*size, *shape, level)))
                .collect(),
        );
    }

    let mut empty = HashMap::<SquareImageSize, Handle<Image>>::new();
    for size in SquareImageSize::iter() {
        empty.insert(*size, image_assets.add(get_empty_square_image(*size)));
    }

    SquareImageAssets::new(images, empty)
}

fn unload_square_image_assets(mut commands: Commands) {
    commands.remove_resource::<SquareImageAssets>();
}

fn setup_screen(
    mut commands: Commands,
    player_data: ResMut<PlayerData>,
    square_image_assets: Res<SquareImageAssets>,
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
                        player_data.rc.board_square_translation(x as i32, y as i32),
                    ),
                    sprite: Sprite {
                        custom_size: Some(player_data.rc.square_size()),
                        ..default()
                    },
                    texture: square_image_assets.get_empty(SquareImageSize::Normal),
                    ..default()
                },
                GameEntityMarker,
                BoardSquareEntityMarker(x, y),
            ));
        }
    }

    commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_translation(player_data.rc.board_cover_translation()),
                sprite: Sprite {
                    color: RED.into(),
                    custom_size: Some(player_data.rc.board_cover_size()),
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
            text: Text::from_sections([
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
            text: Text::from_sections([
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
            text: Text::from_sections([
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
                TextSection::from_style(TextStyle {
                    font_size: player_data.rc.unit(),
                    color: WHITE.into(),
                    ..default()
                });
                10
            ]),
            transform: Transform::from_translation(player_data.rc.statistics_translation()),
            ..default()
        },
        GameEntityMarker,
        StatisticsEntityMarker,
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_sections([
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

    for shape in PieceShape::iter() {
        for square in Piece::new(*shape).to_squares() {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_translation(player_data.rc.piece_count_translation(
                        *shape as usize,
                        square.0,
                        square.1,
                    )),
                    sprite: Sprite {
                        custom_size: Some(player_data.rc.piece_count_square_size()),
                        ..default()
                    },
                    texture: square_image_assets.get_image(SquareImageSize::Small, *shape),
                    ..default()
                },
                GameEntityMarker,
                PieceCountEntityMarker(*shape),
            ));
            commands.spawn((
                Text2dBundle {
                    text: Text::from_sections([TextSection::from_style(TextStyle {
                        font_size: player_data.rc.unit(),
                        color: WHITE.into(),
                        ..default()
                    })]),
                    transform: Transform::from_translation(
                        player_data
                            .rc
                            .piece_count_counter_translation(*shape as usize),
                    ),
                    ..default()
                },
                GameEntityMarker,
                PieceCountCounterEntityMarker(*shape),
            ));
        }
    }

    player_data
        .board
        .get_curr_piece_squares()
        .iter()
        .for_each(|blk| {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_translation(
                        player_data.rc.curr_piece_translation(blk.0, blk.1),
                    ),
                    sprite: Sprite {
                        custom_size: Some(player_data.rc.square_size()),
                        ..default()
                    },
                    texture: square_image_assets.get_image(
                        SquareImageSize::Normal,
                        player_data.board.get_curr_piece().shape(),
                    ),
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
        .to_squares()
        .iter()
        .for_each(|blk| {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_translation(
                        player_data.rc.next_piece_translation(blk.0, blk.1),
                    ),
                    sprite: Sprite {
                        custom_size: Some(player_data.rc.square_size()),
                        ..default()
                    },
                    texture: square_image_assets.get_image(
                        SquareImageSize::Normal,
                        player_data.board.get_next_piece().shape(),
                    ),
                    ..default()
                },
                GameEntityMarker,
                NextPieceEntityMarker,
            ));
        });
}

fn update_statistics_system(
    mut query: ParamSet<(
        Query<&mut Text, With<LinesEntityMarker>>,
        Query<&mut Text, With<ScoreEntityMarker>>,
        Query<&mut Text, With<LevelEntityMarker>>,
        Query<&mut Text, With<StatisticsEntityMarker>>,
        Query<&mut Text, With<DASEntityMarker>>,
        Query<(&mut Text, &PieceCountCounterEntityMarker)>,
    )>,
    player_data: ResMut<PlayerData>,
) {
    if let Ok(mut text) = query.p0().get_single_mut() {
        text.sections[1].value = format!("{:03}", player_data.board.lines());
    }
    if let Ok(mut text) = query.p1().get_single_mut() {
        text.sections[1].value = format!("{:07}", player_data.board.score());
    }
    if let Ok(mut text) = query.p2().get_single_mut() {
        text.sections[1].value = format!("{:02}", player_data.board.level());
        text.sections[2].value = format!(" {:02}", player_data.board.start_level());
    }
    if let Ok(mut text) = query.p3().get_single_mut() {
        text.sections[0].value = format!("BRN {:4}\n", player_data.board.burned_lines());
        text.sections[1].value = format!(" 1X {:4}\n", player_data.board.single());
        text.sections[2].value = format!(" 2X {:4}\n", player_data.board.double());
        text.sections[3].value = format!(" 3X {:4}\n", player_data.board.triple());
        text.sections[4].value = format!("TRT {:4}\n", player_data.board.tetris());
        text.sections[5].value = format!("TRT ");
        let rate = (player_data.board.tetris_rate() * 100.0).round() as usize;
        text.sections[6].value = format!("{:3}%\n", rate);
        match rate {
            0..50 => text.sections[6].style.color = RED.into(),
            50..80 => text.sections[6].style.color = YELLOW.into(),
            _ => text.sections[6].style.color = GREEN.into(),
        }
        text.sections[7].value = format!("DRT ");
        let drought = player_data.board.drought();
        text.sections[8].value = format!("{:02}", drought);
        match drought {
            0..7 => text.sections[8].style.color = WHITE.into(),
            7..14 => text.sections[8].style.color = YELLOW.into(),
            _ => text.sections[8].style.color = RED.into(),
        }
        text.sections[9].value = format!(" ({:02})\n", player_data.board.max_drought());
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
    for (mut text, shape) in query.p5().iter_mut() {
        text.sections[0].value = format!("{:03}", player_data.board.get_piece_count(shape.0));
    }
}

mod state_game_running {
    use super::*;

    pub(super) fn tick_system(time: Res<Time>, mut player_data: ResMut<PlayerData>) {
        player_data.game_timer.tick(time.delta());
        player_data.press_down_timer.tick(time.delta());
    }

    pub(super) fn handle_input_system(
        time: Res<Time>,
        player_inputs: Res<PlayerInputs>,
        mut query: ParamSet<(
            Query<(&mut Transform, &mut Handle<Image>), With<CurrPieceEntityMarker>>,
            Query<&mut Visibility, With<BoardCoverEntityMarker>>,
        )>,
        mut e_play_sound: EventWriter<PlaySoundEvent>,
        mut player_data: ResMut<PlayerData>,
        mut player_state: ResMut<NextState<PlayerState>>,
        square_image_assets: Res<SquareImageAssets>,
    ) {
        if player_inputs.start {
            *query.p1().single_mut() = Visibility::Inherited;
            player_state.set(PlayerState::GamePause);
            return;
        }

        let (moved, lr_moved, rotated) = handle_input(&player_inputs, &time, &mut player_data);
        if moved {
            std::iter::zip(
                query.p0().iter_mut(),
                player_data.board.get_curr_piece_squares(),
            )
            .for_each(|((mut transform, mut image), blk)| {
                *image = square_image_assets.get_image(
                    SquareImageSize::Normal,
                    player_data.board.get_curr_piece().shape(),
                );
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
        inputs: &PlayerInputs,
        time: &Time,
        player_data: &mut PlayerData,
    ) -> (bool, bool, bool) {
        let mut down_moved = false;
        let mut lr_moved = false;
        let mut rotated = false;

        if player_data.can_press_down {
            if inputs.down.1 {
                if player_data.press_down_timer.commit() {
                    down_moved |= player_data.board.move_piece_down();
                    player_data.lock_curr_piece_immediately = !down_moved;
                }
            } else {
                player_data.can_press_down = false;
            }
        } else if inputs.down.0 {
            player_data.can_press_down = true;
            player_data.game_timer.reset();
            player_data.fall_tick.set_level(player_data.board.level());
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

        if inputs.a.0 {
            rotated |= player_data.board.rotate_piece_clockwise();
        }
        if inputs.b.0 {
            rotated |= player_data.board.rotate_piece_counter_clockwise();
        }

        (down_moved | lr_moved | rotated, lr_moved, rotated)
    }

    pub(super) fn curr_piece_fall_system(
        mut query: ParamSet<(
            Query<(&mut Handle<Image>, &BoardSquareEntityMarker)>,
            Query<(&mut Transform, &mut Handle<Image>), With<CurrPieceEntityMarker>>,
        )>,
        mut e_play_sound: EventWriter<PlaySoundEvent>,
        mut player_data: ResMut<PlayerData>,
        mut player_state: ResMut<NextState<PlayerState>>,
        square_image_assets: Res<SquareImageAssets>,
    ) {
        let lock = {
            if std::mem::replace(&mut player_data.lock_curr_piece_immediately, false) {
                player_data.game_timer.reset();
                true
            } else {
                let threshold = player_data.fall_tick.threshold();
                player_data.game_timer.commit(threshold)
            }
        };
        if lock {
            let new_level = player_data.board.level();
            player_data.fall_tick.set_level(new_level);

            if player_data.board.move_piece_down() {
                std::iter::zip(
                    query.p1().iter_mut(),
                    player_data.board.get_curr_piece_squares(),
                )
                .for_each(|((mut transform, mut image), blk)| {
                    *image = square_image_assets.get_image(
                        SquareImageSize::Normal,
                        player_data.board.get_curr_piece().shape(),
                    );
                    transform.translation = player_data.rc.curr_piece_translation(blk.0, blk.1);
                });
            } else if !player_data.board.is_curr_position_valid() {
                e_play_sound.send(PlaySoundEvent::GameOver);
                player_state.set(PlayerState::GameOver);
            } else {
                player_data.can_press_down = false; // keep pressing down will not affect next piece

                let min_y = player_data
                    .board
                    .get_curr_piece_squares()
                    .iter()
                    .fold(19, |acc, blk| acc.min(blk.1 as u64));
                player_data.entry_delay_tick = EntryDelayTick::new(min_y);

                player_data.board.lock_curr_piece();
                query.p1().iter_mut().for_each(|(mut transform, _)| {
                    // make invisible
                    transform.translation.z = player_data.rc.board_translation().z - 1.0;
                });

                query.p0().iter_mut().for_each(|(mut image, coordinate)| {
                    *image = square_image_assets.get_image_or_empty(
                        SquareImageSize::Normal,
                        player_data
                            .board
                            .get_square(coordinate.0 as i32, coordinate.1 as i32),
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

    pub(super) fn tick_system(
        time: Res<Time>,
        mut query: Query<(&mut Handle<Image>, &BoardSquareEntityMarker)>,
        mut e_play_sound: EventWriter<PlaySoundEvent>,
        mut player_data: ResMut<PlayerData>,
        mut player_state: ResMut<NextState<PlayerState>>,
        square_image_assets: Res<SquareImageAssets>,
    ) {
        player_data.game_timer.tick(time.delta());
        let threshold = player_data.line_clear_tick.threshold();
        if player_data.game_timer.commit(threshold) {
            if let Some((left, right)) = player_data.line_clear_phase.next_cols() {
                for (mut image, coordinate) in query.iter_mut() {
                    if (coordinate.0 == left || coordinate.0 == right)
                        && player_data.line_clear_rows.contains(&coordinate.1)
                    {
                        *image = square_image_assets.get_empty(SquareImageSize::Normal);
                    }
                }
            } else {
                if player_data.board.clear_lines() {
                    e_play_sound.send(PlaySoundEvent::LevelUp);
                }
                let new_level = player_data.board.level();
                player_data.fall_tick.set_level(new_level);
                player_state.set(PlayerState::GameUpdateSquareImageAssets);
            }
        }
    }
}

mod state_game_update_assets {
    use super::*;

    pub(super) fn update_square_image_assets(
        mut player_state: ResMut<NextState<PlayerState>>,
        mut image_assets: ResMut<Assets<Image>>,
        mut square_image_assets: ResMut<SquareImageAssets>,
        player_data: Res<PlayerData>,
    ) {
        *square_image_assets =
            load_square_image_assets_impl(player_data.board.level(), &mut image_assets);
        player_state.set(PlayerState::GameEntryDelay);
    }
}

mod state_game_entry_delay {
    use super::*;

    pub(super) fn tick_system(
        time: Res<Time>,
        mut query: ParamSet<(
            Query<(&mut Handle<Image>, &BoardSquareEntityMarker)>,
            Query<(&mut Transform, &mut Handle<Image>), With<CurrPieceEntityMarker>>,
            Query<(&mut Transform, &mut Handle<Image>), With<NextPieceEntityMarker>>,
            Query<(&mut Handle<Image>, &PieceCountEntityMarker)>,
        )>,
        mut player_data: ResMut<PlayerData>,
        mut player_state: ResMut<NextState<PlayerState>>,
        square_image_assets: Res<SquareImageAssets>,
    ) {
        player_data.game_timer.tick(time.delta());
        let threshold = player_data.entry_delay_tick.threshold();
        if player_data.game_timer.commit(threshold) {
            player_data.board.switch_to_next_piece();

            query.p0().iter_mut().for_each(|(mut image, coordinate)| {
                *image = square_image_assets.get_image_or_empty(
                    SquareImageSize::Normal,
                    player_data
                        .board
                        .get_square(coordinate.0 as i32, coordinate.1 as i32),
                );
            });

            std::iter::zip(
                query.p1().iter_mut(),
                player_data.board.get_curr_piece_squares(),
            )
            .for_each(|((mut transform, mut image), blk)| {
                *image = square_image_assets.get_image(
                    SquareImageSize::Normal,
                    player_data.board.get_curr_piece().shape(),
                );
                transform.translation = player_data.rc.curr_piece_translation(blk.0, blk.1);
            });
            std::iter::zip(
                query.p2().iter_mut(),
                player_data.board.get_next_piece().to_squares(),
            )
            .for_each(|((mut transform, mut image), blk)| {
                *image = square_image_assets.get_image(
                    SquareImageSize::Normal,
                    player_data.board.get_next_piece().shape(),
                );
                transform.translation = player_data.rc.next_piece_translation(blk.0, blk.1);
            });
            query.p3().iter_mut().for_each(|(mut image, shape)| {
                *image = square_image_assets.get_image(SquareImageSize::Small, shape.0);
            });

            player_state.set(PlayerState::GameRunning);
        }
    }
}

mod state_game_pause {
    use super::*;

    pub(super) fn handle_input_system(
        player_inputs: Res<PlayerInputs>,
        mut query: ParamSet<(Query<&mut Visibility, With<BoardCoverEntityMarker>>,)>,
        mut player_state: ResMut<NextState<PlayerState>>,
    ) {
        if player_inputs.start {
            *query.p0().single_mut() = Visibility::Hidden;
            player_state.set(PlayerState::GameRunning);
        }
    }
}

mod state_game_over {
    use super::*;

    pub(super) fn handle_input_system(
        player_inputs: Res<PlayerInputs>,
        mut app_state: ResMut<NextState<AppState>>,
    ) {
        if player_inputs.start {
            app_state.set(AppState::LevelMenu);
        }
    }
}
