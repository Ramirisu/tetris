use bevy::{
    color::palettes::css::{BLACK, GREEN, RED, WHITE, YELLOW},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

use crate::{
    app_state::AppState,
    audio::plugin::PlaySoundEvent,
    controller::Controller,
    input::{controller_mapping::ControllerMapping, player_inputs::PlayerInputs},
    utility::{despawn_all, format_hhmmss},
};

use super::{
    asset::{ColorMaterialAssets, SquareImageAssets},
    board::Board,
    das_counter::DASCounter,
    game::{GameConfig, GameState},
    palette::SquareImageSize,
    piece::Piece,
    player::{LineClearPhase, PlayerData, PlayerPhase},
    transform::GameTransform,
};

pub fn setup(app: &mut App) {
    app.insert_resource(GameTransform::default())
        .init_state::<GameState>()
        .insert_resource(GameConfig::default())
        .insert_resource(PlayerData::default())
        .init_state::<PlayerPhase>()
        .add_systems(OnEnter(AppState::Game), (load_assets, setup_screen).chain())
        .add_systems(
            OnExit(AppState::Game),
            (despawn_all::<GameEntityMarker>, unload_assets),
        )
        .add_systems(
            Update,
            (
                (
                    (
                        increase_stopwatch_system,
                        state_player_dropping::tick_system,
                        state_player_dropping::handle_input_system,
                        state_player_dropping::curr_piece_fall_system,
                        update_statistics_system,
                    )
                        .chain()
                        .run_if(in_state(PlayerPhase::Dropping)),
                    (
                        increase_stopwatch_system,
                        state_player_line_clear::tick_system,
                        update_statistics_system,
                    )
                        .chain()
                        .run_if(in_state(PlayerPhase::LineClear)),
                    (
                        increase_stopwatch_system,
                        state_player_entry_delay::tick_system,
                    )
                        .run_if(in_state(PlayerPhase::EntryDelay)),
                )
                    .run_if(in_state(GameState::Running)),
                state_game_pause::handle_input_system.run_if(in_state(GameState::Pause)),
                state_game_over::handle_input_system.run_if(in_state(GameState::Over)),
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
struct FlashEntityMarker;

#[derive(Component)]
struct BoardCoverEntityMarker;

#[derive(Component)]
struct LinesEntityMarker;

#[derive(Component)]
struct ScoreEntityMarker;

#[derive(Component)]
struct LevelEntityMarker;

#[derive(Component)]
struct DASCounterEntityMarker;

#[derive(Component)]
struct DASIndicatorEntityMarker;

#[derive(Component)]
struct GameModeEntityMarker;

#[derive(Component)]
struct GameStopwatchEntityMarker;

#[derive(Component)]
struct StatisticsEntityMarker;

#[derive(Component)]
struct PieceCountEntityMarker(Piece);

#[derive(Component)]
struct PieceCountCounterEntityMarker(Piece);

#[derive(Component)]
struct CurrPieceEntityMarker;

#[derive(Component)]
struct NextPieceEntityMarker(usize);

#[derive(Component)]
enum PlayerInputsDisplayEntityMarker {
    Left,
    Right,
    Up,
    Down,
    A,
    B,
}

fn load_assets(
    mut commands: Commands,
    mut image_assets: ResMut<Assets<Image>>,
    mut color_material_assets: ResMut<Assets<ColorMaterial>>,
    player_data: Res<PlayerData>,
) {
    commands.insert_resource(SquareImageAssets::new(
        &mut image_assets,
        player_data.board.level(),
    ));
    commands.insert_resource(ColorMaterialAssets {
        red: color_material_assets.add(Color::from(RED)),
        white: color_material_assets.add(Color::from(WHITE)),
    });
}

fn unload_assets(mut commands: Commands) {
    commands.remove_resource::<SquareImageAssets>();
    commands.remove_resource::<ColorMaterialAssets>();
}

fn setup_screen(
    mut commands: Commands,
    player_data: Res<PlayerData>,
    square_image_assets: Res<SquareImageAssets>,
    game_transform: Res<GameTransform>,
    mut meshes: ResMut<Assets<Mesh>>,
    color_materials: Res<ColorMaterialAssets>,
) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(game_transform.flash_translation()),
            sprite: Sprite {
                color: WHITE.into(),
                custom_size: Some(game_transform.flash_size()),
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        },
        GameEntityMarker,
        FlashEntityMarker,
    ));
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(game_transform.board_background_translation()),
            sprite: Sprite {
                color: WHITE.into(),
                custom_size: Some(game_transform.board_background_size()),
                ..default()
            },
            ..default()
        },
        GameEntityMarker,
        DASIndicatorEntityMarker,
    ));
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(game_transform.board_translation()),
            sprite: Sprite {
                color: BLACK.into(),
                custom_size: Some(game_transform.board_size()),
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
                        game_transform.board_square_translation(x as i32, y as i32),
                    ),
                    sprite: Sprite {
                        custom_size: Some(game_transform.square_size()),
                        ..default()
                    },
                    texture: square_image_assets.get_image(
                        SquareImageSize::Normal,
                        player_data.board.get_square(x as i32, y as i32),
                    ),
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
                transform: Transform::from_translation(game_transform.board_cover_translation()),
                sprite: Sprite {
                    color: RED.into(),
                    custom_size: Some(game_transform.board_cover_size()),
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
                        font_size: game_transform.fs_medium(),
                        color: WHITE.into(),
                        ..default()
                    },
                ),
                transform: Transform::from_translation(game_transform.board_cover_translation()),
                ..default()
            });
        });

    commands.spawn((
        Text2dBundle {
            text: Text::from_sections([
                TextSection {
                    value: "LINES\n".into(),
                    style: TextStyle {
                        font_size: game_transform.fs_large(),
                        color: WHITE.into(),
                        ..default()
                    },
                    ..default()
                },
                TextSection::from_style(TextStyle {
                    font_size: game_transform.fs_xlarge(),
                    color: WHITE.into(),
                    ..default()
                }),
            ])
            .with_justify(JustifyText::Center),
            transform: Transform::from_translation(game_transform.lines_translation()),
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
                        font_size: game_transform.fs_large(),
                        color: WHITE.into(),
                        ..default()
                    },
                    ..default()
                },
                TextSection::from_style(TextStyle {
                    font_size: game_transform.fs_xlarge(),
                    color: WHITE.into(),
                    ..default()
                }),
            ])
            .with_justify(JustifyText::Center),
            transform: Transform::from_translation(game_transform.score_translation()),
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
                        font_size: game_transform.fs_large(),
                        color: WHITE.into(),
                        ..default()
                    },
                    ..default()
                },
                TextSection::from_style(TextStyle {
                    font_size: game_transform.fs_xlarge(),
                    color: WHITE.into(),
                    ..default()
                }),
            ]),
            transform: Transform::from_translation(game_transform.level_translation()),
            ..default()
        },
        GameEntityMarker,
        LevelEntityMarker,
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_sections(vec![
                TextSection::from_style(TextStyle {
                    font_size: game_transform.fs_medium(),
                    color: WHITE.into(),
                    ..default()
                });
                10
            ]),
            transform: Transform::from_translation(game_transform.statistics_translation()),
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
                        font_size: game_transform.fs_medium(),
                        color: WHITE.into(),
                        ..default()
                    },
                    ..default()
                },
                TextSection::from_style(TextStyle {
                    font_size: game_transform.fs_large(),
                    color: WHITE.into(),
                    ..default()
                }),
            ]),
            transform: Transform::from_translation(game_transform.das_translation()),
            visibility: player_data.das_counter.get_counter_visibility(),
            ..default()
        },
        GameEntityMarker,
        DASCounterEntityMarker,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_sections(vec![
                TextSection::from_style(TextStyle {
                    font_size: game_transform.fs_medium(),
                    color: WHITE.into(),
                    ..default()
                });
                12
            ]),
            transform: Transform::from_translation(game_transform.game_mode_translation()),
            ..default()
        },
        GameEntityMarker,
        GameModeEntityMarker,
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_sections([
                TextSection {
                    value: "TIME ".into(),
                    style: TextStyle {
                        font_size: game_transform.fs_medium(),
                        color: WHITE.into(),
                        ..default()
                    },
                    ..default()
                },
                TextSection::from_style(TextStyle {
                    font_size: game_transform.fs_medium(),
                    color: WHITE.into(),
                    ..default()
                }),
            ]),
            transform: Transform::from_translation(game_transform.stopwatch_translation()),
            ..default()
        },
        GameEntityMarker,
        GameStopwatchEntityMarker,
    ));

    Piece::iter()
        .filter(|piece| **piece != Piece::X)
        .for_each(|piece| {
            piece
                .get_squares_with_piece_center_align()
                .iter()
                .for_each(|square| {
                    commands.spawn((
                        SpriteBundle {
                            transform: Transform::from_translation(
                                game_transform.piece_count_translation(
                                    piece.variant_index(),
                                    square.0,
                                    square.1,
                                ),
                            ),
                            sprite: Sprite {
                                custom_size: Some(game_transform.piece_count_square_size()),
                                ..default()
                            },
                            texture: square_image_assets.get_image(SquareImageSize::Small, *piece),
                            ..default()
                        },
                        GameEntityMarker,
                        PieceCountEntityMarker(*piece),
                    ));
                    commands.spawn((
                        Text2dBundle {
                            text: Text::from_sections([TextSection::from_style(TextStyle {
                                font_size: game_transform.fs_medium(),
                                color: WHITE.into(),
                                ..default()
                            })]),
                            transform: Transform::from_translation(
                                game_transform
                                    .piece_count_counter_translation(piece.variant_index()),
                            ),
                            ..default()
                        },
                        GameEntityMarker,
                        PieceCountCounterEntityMarker(*piece),
                    ));
                });
        });

    player_data
        .board
        .get_curr_piece_squares()
        .iter()
        .for_each(|sqr| {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_translation(
                        game_transform.curr_piece_translation(sqr.0, sqr.1),
                    ),
                    sprite: Sprite {
                        custom_size: Some(game_transform.square_size()),
                        ..default()
                    },
                    texture: square_image_assets
                        .get_image(SquareImageSize::Normal, player_data.board.get_curr_piece()),
                    ..default()
                },
                GameEntityMarker,
                CurrPieceEntityMarker,
            ));
        });

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "NEXT",
                TextStyle {
                    font_size: game_transform.fs_large(),
                    color: WHITE.into(),
                    ..default()
                },
            ),
            transform: Transform::from_translation(game_transform.next_piece_label_translation()),
            ..default()
        },
        GameEntityMarker,
    ));

    player_data
        .board
        .get_next_pieces()
        .iter()
        .enumerate()
        .for_each(|(index, _)| {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_translation(
                        game_transform.next_piece_slot_background_translation(index),
                    ),
                    sprite: Sprite {
                        color: WHITE.into(),
                        custom_size: Some(game_transform.next_piece_slot_background_size(index)),
                        ..default()
                    },
                    visibility: player_data.next_piece_hint.get_visibility(index),
                    ..default()
                },
                GameEntityMarker,
            ));
        });

    player_data
        .board
        .get_next_pieces()
        .iter()
        .enumerate()
        .for_each(|(index, _)| {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_translation(
                        game_transform.next_piece_slot_translation(index),
                    ),
                    sprite: Sprite {
                        color: BLACK.into(),
                        custom_size: Some(game_transform.next_piece_slot_size(index)),
                        ..default()
                    },
                    visibility: player_data.next_piece_hint.get_visibility(index),
                    ..default()
                },
                GameEntityMarker,
            ));
        });

    player_data
        .board
        .get_next_pieces()
        .iter()
        .enumerate()
        .for_each(|(index, piece)| {
            piece
                .get_squares_with_piece_center_align()
                .iter()
                .for_each(|sqr| {
                    commands.spawn((
                        SpriteBundle {
                            transform: Transform::from_translation(
                                game_transform.next_piece_translation(sqr.0, sqr.1, index),
                            ),
                            sprite: Sprite {
                                custom_size: Some(game_transform.next_piece_square_size(index)),
                                ..default()
                            },
                            texture: square_image_assets.get_image(SquareImageSize::Normal, *piece),
                            visibility: player_data.next_piece_hint.get_visibility(index),
                            ..default()
                        },
                        GameEntityMarker,
                        NextPieceEntityMarker(index),
                    ));
                });
        });

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Rectangle::from_size(game_transform.inputs_rect_size()))
                .into(),
            transform: Transform::from_translation(
                game_transform.inputs_button_center_translation(),
            ),
            material: color_materials.white.clone(),
            ..default()
        },
        GameEntityMarker,
    ));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Rectangle::from_size(game_transform.inputs_rect_size()))
                .into(),
            transform: Transform::from_translation(game_transform.inputs_button_left_translation()),
            material: color_materials.white.clone(),
            ..default()
        },
        GameEntityMarker,
        PlayerInputsDisplayEntityMarker::Left,
    ));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Rectangle::from_size(game_transform.inputs_rect_size()))
                .into(),
            transform: Transform::from_translation(
                game_transform.inputs_button_right_translation(),
            ),
            material: color_materials.white.clone(),
            ..default()
        },
        GameEntityMarker,
        PlayerInputsDisplayEntityMarker::Right,
    ));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Rectangle::from_size(game_transform.inputs_rect_size()))
                .into(),
            transform: Transform::from_translation(game_transform.inputs_button_up_translation()),
            material: color_materials.white.clone(),
            ..default()
        },
        GameEntityMarker,
        PlayerInputsDisplayEntityMarker::Up,
    ));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Rectangle::from_size(game_transform.inputs_rect_size()))
                .into(),
            transform: Transform::from_translation(game_transform.inputs_button_down_translation()),
            material: color_materials.white.clone(),
            ..default()
        },
        GameEntityMarker,
        PlayerInputsDisplayEntityMarker::Down,
    ));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Circle::new(game_transform.inputs_circle_scale()))
                .into(),
            transform: Transform::from_translation(game_transform.inputs_button_a_translation()),
            material: color_materials.white.clone(),
            ..default()
        },
        GameEntityMarker,
        PlayerInputsDisplayEntityMarker::A,
    ));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Circle::new(game_transform.inputs_circle_scale()))
                .into(),
            transform: Transform::from_translation(game_transform.inputs_button_b_translation()),
            material: color_materials.white.clone(),
            ..default()
        },
        GameEntityMarker,
        PlayerInputsDisplayEntityMarker::B,
    ));
}

fn increase_stopwatch_system(time: Res<Time>, mut player_data: ResMut<PlayerData>) {
    player_data.stopwatch.tick(time.delta());
}

fn update_statistics_system(
    mut query: ParamSet<(
        ParamSet<(
            Query<&mut Text, With<LinesEntityMarker>>,
            Query<&mut Text, With<ScoreEntityMarker>>,
            Query<&mut Text, With<LevelEntityMarker>>,
            Query<&mut Text, With<StatisticsEntityMarker>>,
            Query<&mut Text, With<GameModeEntityMarker>>,
            Query<&mut Text, With<GameStopwatchEntityMarker>>,
            Query<(&mut Text, &PieceCountCounterEntityMarker)>,
        )>,
        Query<&mut Text, With<DASCounterEntityMarker>>,
        Query<&mut Sprite, With<DASIndicatorEntityMarker>>,
    )>,
    game_config: Res<GameConfig>,
    player_data: Res<PlayerData>,
) {
    if let Ok(mut text) = query.p0().p0().get_single_mut() {
        text.sections[1].value = format!("{:03}", player_data.board.lines());
    }
    if let Ok(mut text) = query.p0().p1().get_single_mut() {
        text.sections[1].value = format!("{:07}", player_data.board.score());
    }
    if let Ok(mut text) = query.p0().p2().get_single_mut() {
        text.sections[1].value = format!("{:02}", player_data.board.level());
    }
    if let Ok(mut text) = query.p0().p3().get_single_mut() {
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
    if let Ok(mut text) = query.p0().p4().get_single_mut() {
        text.sections[0].value = format!("SLV {:3}\n", game_config.start_level);
        text.sections[1].value = format!("CAP {:3}\n", game_config.linecap.to_string_abbr());
        text.sections[2].value = format!("TRS {:3}\n", game_config.transition.to_string_abbr());
        text.sections[3].value = format!("GRV {:3}\n", game_config.gravity.to_string_abbr());
        text.sections[4].value = format!(" SD {:3}\n", game_config.seed.to_string_abbr());
        text.sections[5].value = format!("TVS {:3}\n", game_config.tv_system.to_string_abbr());
    }
    if let Ok(mut text) = query.p0().p5().get_single_mut() {
        text.sections[1].value = format_hhmmss(player_data.stopwatch.elapsed());
    }
    for (mut text, piece) in query.p0().p6().iter_mut() {
        text.sections[0].value = format!("{:03}", player_data.board.get_piece_count(piece.0));
    }

    let das_color = if player_data.das_timer.is_active() {
        GREEN
    } else {
        RED
    };
    if let Ok(mut text) = query.p1().get_single_mut() {
        text.sections[1].value = format!(
            "{:02}",
            game_config
                .tv_system
                .duration_to_ticks(player_data.das_timer.elapsed())
        );
        text.sections[1].style.color = das_color.into();
    }
    if player_data.das_counter == DASCounter::Full {
        if let Ok(mut sprite) = query.p2().get_single_mut() {
            sprite.color = das_color.into();
        }
    }
}

mod state_player_dropping {
    use crate::game::timer::EntryDelayTimer;

    use super::*;

    pub(super) fn tick_system(time: Res<Time>, mut player_data: ResMut<PlayerData>) {
        player_data.fall_timer.tick(time.delta());
    }

    pub(super) fn handle_input_system(
        time: Res<Time>,
        keys: Res<ButtonInput<KeyCode>>,
        buttons: Res<ButtonInput<GamepadButton>>,
        controller: Res<Controller>,
        controller_mapping: Res<ControllerMapping>,
        mut query: ParamSet<(
            Query<&mut Transform, With<CurrPieceEntityMarker>>,
            Query<&mut Visibility, With<BoardCoverEntityMarker>>,
            Query<(&mut Handle<ColorMaterial>, &PlayerInputsDisplayEntityMarker)>,
        )>,
        mut play_sound: EventWriter<PlaySoundEvent>,
        mut player_data: ResMut<PlayerData>,
        mut game_state: ResMut<NextState<GameState>>,
        mut app_state: ResMut<NextState<AppState>>,
        game_transform: Res<GameTransform>,
        color_materials: Res<ColorMaterialAssets>,
    ) {
        let player_inputs = PlayerInputs::with_keyboard(&keys)
            | PlayerInputs::with_gamepads(&buttons, &controller, *controller_mapping);

        if player_inputs.soft_reset {
            play_sound.send(PlaySoundEvent::StartGame);
            app_state.set(AppState::Splash);
            return;
        }

        if player_inputs.start.just_pressed {
            *query.p1().single_mut() = Visibility::Inherited;
            game_state.set(GameState::Pause);
            return;
        }

        query.p2().iter_mut().for_each(|(mut color, marker)| {
            let pressed = match marker {
                PlayerInputsDisplayEntityMarker::Left => player_inputs.left.pressed,
                PlayerInputsDisplayEntityMarker::Right => player_inputs.right.pressed,
                PlayerInputsDisplayEntityMarker::Up => player_inputs.up.pressed,
                PlayerInputsDisplayEntityMarker::Down => player_inputs.down.pressed,
                PlayerInputsDisplayEntityMarker::A => player_inputs.a.pressed,
                PlayerInputsDisplayEntityMarker::B => player_inputs.b.pressed,
            };
            if pressed {
                *color = color_materials.red.clone();
            } else {
                *color = color_materials.white.clone();
            }
        });

        let (moved, lr_moved, rotated) = handle_input(&player_inputs, &time, &mut player_data);
        if moved {
            std::iter::zip(
                query.p0().iter_mut(),
                player_data.board.get_curr_piece_squares(),
            )
            .for_each(|(mut transform, sqr)| {
                transform.translation = game_transform.curr_piece_translation(sqr.0, sqr.1);
            });
        }
        if lr_moved {
            play_sound.send(PlaySoundEvent::MoveCurrPiece);
        }
        if rotated {
            play_sound.send(PlaySoundEvent::RotateCurrPiece);
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
            if inputs.down.pressed {
                if player_data.press_down_timer.tick(time.delta()).consume() {
                    down_moved |= player_data.board.move_piece_down();
                    player_data.lock_curr_piece_immediately = !down_moved;
                }
            } else {
                player_data.can_press_down = false;
            }
        } else if inputs.down.just_pressed {
            player_data.can_press_down = true;
            player_data.fall_timer.set_level(player_data.board.level());
            player_data.press_down_timer.reset();
        }

        if !inputs.down.pressed {
            player_data.press_down_timer.reset();

            if inputs.left.just_pressed || inputs.right.just_pressed {
                player_data.das_timer.reset();
                match (inputs.left.just_pressed, inputs.right.just_pressed) {
                    (true, false) => lr_moved |= player_data.board.move_piece_left(),
                    (false, true) => lr_moved |= player_data.board.move_piece_right(),
                    _ => (),
                }
            } else {
                match (inputs.left.pressed, inputs.right.pressed) {
                    (true, true) => {
                        player_data.das_timer.tick(time.delta());
                    }
                    (true, false) => {
                        if !player_data.board.is_left_movable() {
                            player_data.das_timer.charge();
                        } else if player_data.das_timer.tick(time.delta()).consume() {
                            lr_moved |= player_data.board.move_piece_left();
                        }
                    }
                    (false, true) => {
                        if !player_data.board.is_right_movable() {
                            player_data.das_timer.charge();
                        } else if player_data.das_timer.tick(time.delta()).consume() {
                            lr_moved |= player_data.board.move_piece_right();
                        }
                    }
                    _ => (),
                }
            }
        }

        if inputs.a.just_pressed {
            rotated |= player_data.board.rotate_piece_clockwise();
        }
        if inputs.b.just_pressed {
            rotated |= player_data.board.rotate_piece_counter_clockwise();
        }

        (down_moved | lr_moved | rotated, lr_moved, rotated)
    }

    pub(super) fn curr_piece_fall_system(
        mut query: ParamSet<(
            Query<(&mut Handle<Image>, &BoardSquareEntityMarker)>,
            Query<&mut Transform, With<CurrPieceEntityMarker>>,
        )>,
        mut play_sound: EventWriter<PlaySoundEvent>,
        mut game_state: ResMut<NextState<GameState>>,
        mut player_phase: ResMut<NextState<PlayerPhase>>,
        game_config: Res<GameConfig>,
        mut player_data: ResMut<PlayerData>,
        square_image_assets: Res<SquareImageAssets>,
        game_transform: Res<GameTransform>,
    ) {
        let lock = {
            if std::mem::replace(&mut player_data.lock_curr_piece_immediately, false) {
                player_data.fall_timer.reset();
                true
            } else {
                player_data.fall_timer.consume()
            }
        };
        if lock {
            let new_level = player_data.board.level();
            player_data.fall_timer.set_level(new_level);

            if player_data.board.move_piece_down() {
                std::iter::zip(
                    query.p1().iter_mut(),
                    player_data.board.get_curr_piece_squares(),
                )
                .for_each(|(mut transform, sqr)| {
                    transform.translation = game_transform.curr_piece_translation(sqr.0, sqr.1);
                });
            } else if !player_data.board.is_curr_position_valid() {
                play_sound.send(PlaySoundEvent::GameOver);
                game_state.set(GameState::Over);
                player_phase.set(PlayerPhase::Over);
            } else {
                player_data.can_press_down = false; // keep pressing down will not affect next piece

                let min_y = player_data
                    .board
                    .get_curr_piece_squares()
                    .iter()
                    .fold(19, |acc, sqr| acc.min(sqr.1 as u64));
                player_data.entry_delay_timer = EntryDelayTimer::new(min_y, game_config.tv_system);

                player_data.board.lock_curr_piece();
                query.p1().iter_mut().for_each(|mut transform| {
                    // make invisible
                    transform.translation.z = game_transform.board_translation().z - 1.0;
                });

                query.p0().iter_mut().for_each(|(mut image, coordinate)| {
                    *image = square_image_assets.get_image(
                        SquareImageSize::Normal,
                        player_data
                            .board
                            .get_square(coordinate.0 as i32, coordinate.1 as i32),
                    );
                });

                let lines = player_data.board.get_line_clear_rows();
                match lines.len() {
                    0 => {
                        play_sound.send(PlaySoundEvent::LockCurrPiece);
                    }
                    1 | 2 | 3 => {
                        play_sound.send(PlaySoundEvent::LineClear);
                    }
                    4 => {
                        play_sound.send(PlaySoundEvent::TetrisClear);
                    }
                    _ => (),
                }
                if lines.len() > 0 {
                    player_data.line_clear_rows = lines;
                    player_data.line_clear_phase = LineClearPhase::new(game_config.tv_system);
                    player_phase.set(PlayerPhase::LineClear);
                } else {
                    player_phase.set(PlayerPhase::EntryDelay);
                }
            }
        }
    }
}

mod state_player_line_clear {
    use super::*;

    pub(super) fn tick_system(
        time: Res<Time>,
        mut query: ParamSet<(
            Query<(&mut Handle<Image>, &BoardSquareEntityMarker)>,
            Query<&mut Visibility, With<FlashEntityMarker>>,
        )>,
        mut play_sound: EventWriter<PlaySoundEvent>,
        mut player_data: ResMut<PlayerData>,
        mut player_phase: ResMut<NextState<PlayerPhase>>,
        mut square_image_assets: ResMut<SquareImageAssets>,
        mut image_assets: ResMut<Assets<Image>>,
    ) {
        if player_data
            .line_clear_phase
            .timer
            .tick(time.delta())
            .consume()
        {
            let mut to_next_state = true;
            if let Some((left, right, end)) = player_data.line_clear_phase.next() {
                to_next_state = end;
                for (mut image, coordinate) in query.p0().iter_mut() {
                    if (coordinate.0 == left || coordinate.0 == right)
                        && player_data.line_clear_rows.contains(&coordinate.1)
                    {
                        *image = square_image_assets.get_image(SquareImageSize::Normal, Piece::X);
                    }
                }
                if player_data.line_clear_rows.len() == 4 {
                    if let Ok(mut visibility) = query.p1().get_single_mut() {
                        match *visibility {
                            Visibility::Hidden => *visibility = Visibility::Inherited,
                            Visibility::Inherited => *visibility = Visibility::Hidden,
                            _ => (),
                        }
                    }
                }
            }
            if to_next_state {
                let old_level = player_data.board.level();
                player_data.board.clear_lines();
                let new_level = player_data.board.level();
                if new_level > old_level {
                    play_sound.send(PlaySoundEvent::LevelUp);
                    player_data.fall_timer.set_level(new_level);
                    *square_image_assets =
                        SquareImageAssets::new(&mut image_assets, player_data.board.level());
                }
                player_phase.set(PlayerPhase::EntryDelay);
            }
        }
    }
}

mod state_player_entry_delay {
    use super::*;

    pub(super) fn tick_system(
        time: Res<Time>,
        mut query: ParamSet<(
            Query<&mut Visibility, With<FlashEntityMarker>>,
            Query<(&mut Handle<Image>, &BoardSquareEntityMarker)>,
            Query<(&mut Transform, &mut Handle<Image>), With<CurrPieceEntityMarker>>,
            Query<(&mut Transform, &mut Handle<Image>, &NextPieceEntityMarker)>,
            Query<(&mut Handle<Image>, &PieceCountEntityMarker)>,
        )>,
        mut player_data: ResMut<PlayerData>,
        mut player_phase: ResMut<NextState<PlayerPhase>>,
        square_image_assets: Res<SquareImageAssets>,
        game_transform: Res<GameTransform>,
    ) {
        if player_data.entry_delay_timer.tick(time.delta()).consume() {
            player_data.board.switch_to_next_piece();

            if let Ok(mut visibility) = query.p0().get_single_mut() {
                *visibility = Visibility::Hidden;
            }

            query.p1().iter_mut().for_each(|(mut image, coordinate)| {
                *image = square_image_assets.get_image(
                    SquareImageSize::Normal,
                    player_data
                        .board
                        .get_square(coordinate.0 as i32, coordinate.1 as i32),
                );
            });

            std::iter::zip(
                query.p2().iter_mut(),
                player_data.board.get_curr_piece_squares(),
            )
            .for_each(|((mut transform, mut image), sqr)| {
                *image = square_image_assets
                    .get_image(SquareImageSize::Normal, player_data.board.get_curr_piece());
                transform.translation = game_transform.curr_piece_translation(sqr.0, sqr.1);
            });
            std::iter::zip(
                query.p3().iter_mut(),
                player_data
                    .board
                    .get_next_pieces()
                    .iter()
                    .flat_map(|piece| piece.get_squares_with_piece_center_align())
                    .collect::<Vec<_>>(),
            )
            .for_each(|((mut transform, mut image, index), sqr)| {
                *image = square_image_assets.get_image(
                    SquareImageSize::Normal,
                    player_data.board.get_next_pieces()[index.0],
                );
                transform.translation =
                    game_transform.next_piece_translation(sqr.0, sqr.1, index.0);
            });
            query.p4().iter_mut().for_each(|(mut image, piece)| {
                *image = square_image_assets.get_image(SquareImageSize::Small, piece.0);
            });

            player_phase.set(PlayerPhase::Dropping);
        }
    }
}

mod state_game_pause {
    use super::*;

    pub(super) fn handle_input_system(
        keys: Res<ButtonInput<KeyCode>>,
        buttons: Res<ButtonInput<GamepadButton>>,
        controller: Res<Controller>,
        controller_mapping: Res<ControllerMapping>,
        mut query: Query<&mut Visibility, With<BoardCoverEntityMarker>>,
        mut play_sound: EventWriter<PlaySoundEvent>,
        mut game_state: ResMut<NextState<GameState>>,
        mut app_state: ResMut<NextState<AppState>>,
    ) {
        let player_inputs = PlayerInputs::with_keyboard(&keys)
            | PlayerInputs::with_gamepads(&buttons, &controller, *controller_mapping);

        if player_inputs.soft_reset {
            play_sound.send(PlaySoundEvent::StartGame);
            app_state.set(AppState::Splash);
            return;
        }

        if player_inputs.start.just_pressed {
            *query.single_mut() = Visibility::Hidden;
            game_state.set(GameState::Running);
        }
    }
}

mod state_game_over {
    use super::*;

    pub(super) fn handle_input_system(
        keys: Res<ButtonInput<KeyCode>>,
        buttons: Res<ButtonInput<GamepadButton>>,
        controller: Res<Controller>,
        controller_mapping: Res<ControllerMapping>,
        mut play_sound: EventWriter<PlaySoundEvent>,
        mut app_state: ResMut<NextState<AppState>>,
    ) {
        let player_inputs = PlayerInputs::with_keyboard(&keys)
            | PlayerInputs::with_gamepads(&buttons, &controller, *controller_mapping);

        if player_inputs.soft_reset {
            play_sound.send(PlaySoundEvent::StartGame);
            app_state.set(AppState::Splash);
            return;
        }

        if player_inputs.start.just_pressed {
            app_state.set(AppState::LevelMenu);
        }
    }
}
