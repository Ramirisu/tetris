use bevy::{
    color::palettes::css::{BLACK, GREEN, RED, WHITE, YELLOW},
    prelude::*,
};

use crate::{
    app_state::AppState,
    audio::plugin::PlaySoundEvent,
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
    game_config: Res<GameConfig>,
    player_data: Res<PlayerData>,
    square_image_assets: Res<SquareImageAssets>,
    transform: Res<GameTransform>,
    mut meshes: ResMut<Assets<Mesh>>,
    color_materials: Res<ColorMaterialAssets>,
) {
    commands.spawn((
        Sprite {
            color: WHITE.into(),
            custom_size: Some(transform.flash_size()),
            ..default()
        },
        Transform::from_translation(transform.flash_translation()),
        Visibility::Hidden,
        GameEntityMarker,
        FlashEntityMarker,
    ));
    commands.spawn((
        Sprite {
            color: WHITE.into(),
            custom_size: Some(transform.board_background_size()),
            ..default()
        },
        Transform::from_translation(transform.board_background_translation()),
        GameEntityMarker,
        DASIndicatorEntityMarker,
    ));
    commands.spawn((
        Sprite {
            color: BLACK.into(),
            custom_size: Some(transform.board_size()),
            ..default()
        },
        Transform::from_translation(transform.board_translation()),
        GameEntityMarker,
    ));

    for y in 0..Board::BOARD_ROWS {
        for x in 0..Board::BOARD_COLS {
            commands.spawn((
                Sprite {
                    image: square_image_assets.get_image(
                        SquareImageSize::Standard,
                        player_data.board.get_square(x as i32, y as i32),
                    ),
                    custom_size: Some(transform.square_size()),
                    ..default()
                },
                Transform::from_translation(transform.board_square_translation(x as i32, y as i32)),
                GameEntityMarker,
                BoardSquareEntityMarker(x, y),
            ));
        }
    }

    commands
        .spawn((
            Sprite {
                color: RED.into(),
                custom_size: Some(transform.board_cover_size()),
                ..default()
            },
            Transform::from_translation(transform.board_cover_translation()),
            Visibility::Hidden,
            GameEntityMarker,
            BoardCoverEntityMarker,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text2d::new("PRESS START\nTO CONTINUE"),
                TextFont::from_font_size(transform.fs_medium()),
                TextColor::from(WHITE),
                Transform::from_translation(transform.board_cover_translation()),
            ));
        });

    commands
        .spawn((
            Text2d::new("LINES\n"),
            TextFont::from_font_size(transform.fs_large()),
            TextColor::from(WHITE),
            TextLayout::new_with_justify(JustifyText::Center),
            Transform::from_translation(transform.lines_translation()),
            GameEntityMarker,
            LinesEntityMarker,
        ))
        .with_child((
            TextSpan::default(),
            TextFont::from_font_size(transform.fs_xlarge()),
            TextColor::from(WHITE),
        ));
    commands
        .spawn((
            Text2d::new("SCORE\n"),
            TextFont::from_font_size(transform.fs_large()),
            TextColor::from(WHITE),
            TextLayout::new_with_justify(JustifyText::Center),
            Transform::from_translation(transform.score_translation()),
            GameEntityMarker,
            ScoreEntityMarker,
        ))
        .with_child((
            TextSpan::default(),
            TextFont::from_font_size(transform.fs_xlarge()),
            TextColor::from(WHITE),
        ));
    commands
        .spawn((
            Text2d::new("LEVEL  "),
            TextFont::from_font_size(transform.fs_large()),
            TextColor::from(WHITE),
            Transform::from_translation(transform.level_translation()),
            GameEntityMarker,
            LevelEntityMarker,
        ))
        .with_child((
            TextSpan::default(),
            TextFont::from_font_size(transform.fs_xlarge()),
            TextColor::from(WHITE),
        ));
    commands
        .spawn((
            Text2d::default(),
            TextFont::from_font_size(transform.fs_medium()),
            TextColor::from(WHITE),
            Transform::from_translation(transform.statistics_translation()),
            GameEntityMarker,
            StatisticsEntityMarker,
        ))
        .with_children(|parent| {
            for _ in 0..9 {
                parent.spawn((
                    TextSpan::default(),
                    TextFont::from_font_size(transform.fs_medium()),
                    TextColor::from(WHITE),
                ));
            }
        });
    commands
        .spawn((
            Text2d::new("DAS "),
            TextFont::from_font_size(transform.fs_medium()),
            TextColor::from(WHITE),
            Transform::from_translation(transform.das_translation()),
            Visibility::from(game_config.das_counter.into()),
            GameEntityMarker,
            DASCounterEntityMarker,
        ))
        .with_child((
            TextSpan::default(),
            TextFont::from_font_size(transform.fs_large()),
            TextColor::from(WHITE),
        ));

    commands
        .spawn((
            Text2d::default(),
            TextFont::from_font_size(transform.fs_medium()),
            TextColor::from(WHITE),
            Transform::from_translation(transform.game_mode_translation()),
            GameEntityMarker,
            GameModeEntityMarker,
        ))
        .with_children(|parent| {
            for _ in 0..7 {
                parent.spawn((
                    TextSpan::default(),
                    TextFont::from_font_size(transform.fs_medium()),
                    TextColor::from(WHITE),
                ));
            }
        });
    commands.spawn((
        Text2d::default(),
        TextFont::from_font_size(transform.fs_medium()),
        TextColor::from(WHITE),
        Transform::from_translation(transform.stopwatch_translation()),
        GameEntityMarker,
        GameStopwatchEntityMarker,
    ));

    Piece::iter()
        .filter(|piece| **piece != Piece::X)
        .for_each(|piece| {
            piece
                .to_squares_with_piece_center_align()
                .iter()
                .for_each(|sqr| {
                    commands.spawn((
                        Sprite {
                            image: square_image_assets.get_image(SquareImageSize::Small, *piece),
                            custom_size: Some(transform.piece_count_square_size()),
                            ..default()
                        },
                        Transform::from_translation(transform.piece_count_translation(
                            piece.variant_index(),
                            sqr.0,
                            sqr.1,
                        )),
                        GameEntityMarker,
                        PieceCountEntityMarker(*piece),
                    ));
                    commands.spawn((
                        Text2d::default(),
                        TextFont::from_font_size(transform.fs_medium()),
                        TextColor::from(WHITE),
                        Transform::from_translation(
                            transform.piece_count_counter_translation(piece.variant_index()),
                        ),
                        GameEntityMarker,
                        PieceCountCounterEntityMarker(*piece),
                    ));
                });
        });

    player_data
        .board
        .curr_piece_to_squares_with_pos()
        .iter()
        .for_each(|sqr| {
            commands.spawn((
                Sprite {
                    image: square_image_assets
                        .get_image(SquareImageSize::Standard, *player_data.board.curr_piece()),
                    custom_size: Some(transform.square_size()),
                    ..default()
                },
                Transform::from_translation(transform.curr_piece_translation(sqr.0, sqr.1)),
                GameEntityMarker,
                CurrPieceEntityMarker,
            ));
        });

    commands.spawn((
        Text2d::new("NEXT"),
        TextFont::from_font_size(transform.fs_large()),
        TextColor::from(WHITE),
        Transform::from_translation(transform.next_piece_label_translation()),
        GameEntityMarker,
    ));

    player_data
        .board
        .next_pieces()
        .iter()
        .enumerate()
        .for_each(|(index, _)| {
            commands.spawn((
                Sprite {
                    color: WHITE.into(),
                    custom_size: Some(transform.next_piece_slot_background_size(index)),
                    ..default()
                },
                Transform::from_translation(
                    transform.next_piece_slot_background_translation(index),
                ),
                Visibility::from(game_config.next_piece_hint.as_visibility(index)),
                GameEntityMarker,
            ));
        });

    player_data
        .board
        .next_pieces()
        .iter()
        .enumerate()
        .for_each(|(index, _)| {
            commands.spawn((
                Sprite {
                    color: BLACK.into(),
                    custom_size: Some(transform.next_piece_slot_size(index)),
                    ..default()
                },
                Transform::from_translation(transform.next_piece_slot_translation(index)),
                Visibility::from(game_config.next_piece_hint.as_visibility(index)),
                GameEntityMarker,
            ));
        });

    player_data
        .board
        .next_pieces()
        .iter()
        .enumerate()
        .for_each(|(index, piece)| {
            piece
                .to_squares_with_piece_center_align()
                .iter()
                .for_each(|sqr| {
                    commands.spawn((
                        Sprite {
                            image: square_image_assets.get_image(SquareImageSize::Standard, *piece),
                            custom_size: Some(transform.next_piece_square_size(index)),
                            ..default()
                        },
                        Transform::from_translation(
                            transform.next_piece_translation(sqr.0, sqr.1, index),
                        ),
                        Visibility::from(game_config.next_piece_hint.as_visibility(index)),
                        GameEntityMarker,
                        NextPieceEntityMarker(index),
                    ));
                });
        });

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::from_size(transform.inputs_rect_size()))),
        MeshMaterial2d(color_materials.white.clone()),
        Transform::from_translation(transform.inputs_button_center_translation()),
        GameEntityMarker,
    ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::from_size(transform.inputs_rect_size()))),
        MeshMaterial2d(color_materials.white.clone()),
        Transform::from_translation(transform.inputs_button_left_translation()),
        GameEntityMarker,
        PlayerInputsDisplayEntityMarker::Left,
    ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::from_size(transform.inputs_rect_size()))),
        MeshMaterial2d(color_materials.white.clone()),
        Transform::from_translation(transform.inputs_button_right_translation()),
        GameEntityMarker,
        PlayerInputsDisplayEntityMarker::Right,
    ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::from_size(transform.inputs_rect_size()))),
        MeshMaterial2d(color_materials.white.clone()),
        Transform::from_translation(transform.inputs_button_up_translation()),
        GameEntityMarker,
        PlayerInputsDisplayEntityMarker::Up,
    ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::from_size(transform.inputs_rect_size()))),
        MeshMaterial2d(color_materials.white.clone()),
        Transform::from_translation(transform.inputs_button_down_translation()),
        GameEntityMarker,
        PlayerInputsDisplayEntityMarker::Down,
    ));
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(transform.inputs_circle_scale()))),
        MeshMaterial2d(color_materials.white.clone()),
        Transform::from_translation(transform.inputs_button_a_translation()),
        GameEntityMarker,
        PlayerInputsDisplayEntityMarker::A,
    ));
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(transform.inputs_circle_scale()))),
        MeshMaterial2d(color_materials.white.clone()),
        Transform::from_translation(transform.inputs_button_b_translation()),
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
            Query<Entity, With<LinesEntityMarker>>,
            Query<Entity, With<ScoreEntityMarker>>,
            Query<Entity, With<LevelEntityMarker>>,
            Query<Entity, With<StatisticsEntityMarker>>,
            Query<Entity, With<GameModeEntityMarker>>,
            Query<Entity, With<GameStopwatchEntityMarker>>,
            Query<(Entity, &PieceCountCounterEntityMarker)>,
            Query<Entity, With<DASCounterEntityMarker>>,
        )>,
        Query<&mut Sprite, With<DASIndicatorEntityMarker>>,
    )>,
    mut tw: Text2dWriter,
    game_config: Res<GameConfig>,
    player_data: Res<PlayerData>,
) {
    if let Ok(entity) = query.p0().p0().get_single_mut() {
        *tw.text(entity, 1) = format!("{:03}", player_data.board.lines());
    }
    if let Ok(entity) = query.p0().p1().get_single_mut() {
        *tw.text(entity, 1) = game_config.scoring.format(player_data.board.score());
    }
    if let Ok(entity) = query.p0().p2().get_single_mut() {
        *tw.text(entity, 1) = format!("{:02}", player_data.board.level());
    }
    if let Ok(entity) = query.p0().p3().get_single_mut() {
        *tw.text(entity, 0) = format!("BRN {:4}\n", player_data.board.burned_lines());
        *tw.text(entity, 1) = format!(" 1X {:4}\n", player_data.board.single_clear());
        *tw.text(entity, 2) = format!(" 2X {:4}\n", player_data.board.double_clear());
        *tw.text(entity, 3) = format!(" 3X {:4}\n", player_data.board.triple_clear());
        *tw.text(entity, 4) = format!("TRT {:4}\n", player_data.board.tetris_clear());
        *tw.text(entity, 5) = format!("TRT ");
        let rate = (player_data.board.tetris_rate() * 100.0).round() as usize;
        *tw.text(entity, 6) = format!("{:3}%\n", rate);
        match rate {
            0..50 => *tw.color(entity, 6) = RED.into(),
            50..80 => *tw.color(entity, 6) = YELLOW.into(),
            _ => *tw.color(entity, 6) = GREEN.into(),
        }
        *tw.text(entity, 7) = format!("DRT ");
        let drought = player_data.board.drought();
        *tw.text(entity, 8) = format!("{:02}", drought);
        match drought {
            0..7 => *tw.color(entity, 8) = WHITE.into(),
            7..14 => *tw.color(entity, 8) = YELLOW.into(),
            _ => *tw.color(entity, 8) = RED.into(),
        }
        *tw.text(entity, 9) = format!(" ({:02})\n", player_data.board.max_drought());
    }
    if let Ok(entity) = query.p0().p4().get_single_mut() {
        *tw.text(entity, 0) = format!("SLV {:3}\n", game_config.start_level);
        *tw.text(entity, 1) = format!("CAP {:3}\n", game_config.linecap.to_string_abbr());
        *tw.text(entity, 2) = format!("TRS {:3}\n", game_config.transition.to_string_abbr());
        *tw.text(entity, 3) = format!("GRV {:3}\n", game_config.gravity.to_string_abbr());
        *tw.text(entity, 4) = format!("TVS {:3}\n", game_config.tv_system.to_string_abbr());
        *tw.text(entity, 5) = format!("INV {:3}\n", game_config.invisible.to_string_abbr());
        *tw.text(entity, 6) = format!("SDG {:3}\n", game_config.seeding.to_string_abbr());
        *tw.text(entity, 7) = format!("{}\n", player_data.board.seed());
    }
    if let Ok(entity) = query.p0().p5().get_single_mut() {
        *tw.text(entity, 0) = format!("TIME: {}", format_hhmmss(player_data.stopwatch.elapsed()));
    }
    for (entity, piece) in query.p0().p6().iter_mut() {
        *tw.text(entity, 0) = format!("{:03}", player_data.board.get_piece_count(piece.0));
    }

    let das_color = if player_data.das_timer.is_active() {
        GREEN
    } else {
        RED
    };
    if let Ok(entity) = query.p0().p7().get_single_mut() {
        *tw.text(entity, 1) = format!(
            "{:02}",
            game_config
                .tv_system
                .duration_to_ticks(player_data.das_timer.elapsed())
        );
        *tw.color(entity, 1) = das_color.into();
    }
    if game_config.das_counter == DASCounter::Full {
        if let Ok(mut sprite) = query.p1().get_single_mut() {
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
        gamepads: Query<&Gamepad>,
        controller_mapping: Res<ControllerMapping>,
        mut query: ParamSet<(
            Query<&mut Transform, With<CurrPieceEntityMarker>>,
            Query<&mut Visibility, With<BoardCoverEntityMarker>>,
            Query<(
                &mut MeshMaterial2d<ColorMaterial>,
                &PlayerInputsDisplayEntityMarker,
            )>,
        )>,
        mut play_sound: EventWriter<PlaySoundEvent>,
        mut player_data: ResMut<PlayerData>,
        mut game_state: ResMut<NextState<GameState>>,
        mut app_state: ResMut<NextState<AppState>>,
        transform: Res<GameTransform>,
        color_materials: Res<ColorMaterialAssets>,
    ) {
        let player_inputs = PlayerInputs::with_keyboard(&keys)
            | PlayerInputs::with_gamepads(gamepads, *controller_mapping);

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
                color.0 = color_materials.red.clone();
            } else {
                color.0 = color_materials.white.clone();
            }
        });

        let (moved, lr_moved, rotated) = handle_input(&player_inputs, &time, &mut player_data);
        if moved {
            std::iter::zip(
                query.p0().iter_mut(),
                player_data.board.curr_piece_to_squares_with_pos(),
            )
            .for_each(|(mut tf, sqr)| {
                tf.translation = transform.curr_piece_translation(sqr.0, sqr.1);
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
            Query<(&mut Sprite, &mut Visibility, &BoardSquareEntityMarker)>,
            Query<&mut Transform, With<CurrPieceEntityMarker>>,
        )>,
        mut play_sound: EventWriter<PlaySoundEvent>,
        mut game_state: ResMut<NextState<GameState>>,
        mut player_phase: ResMut<NextState<PlayerPhase>>,
        game_config: Res<GameConfig>,
        mut player_data: ResMut<PlayerData>,
        square_image_assets: Res<SquareImageAssets>,
        transform: Res<GameTransform>,
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
                    player_data.board.curr_piece_to_squares_with_pos(),
                )
                .for_each(|(mut tf, sqr)| {
                    tf.translation = transform.curr_piece_translation(sqr.0, sqr.1);
                });
            } else if !player_data.board.is_curr_position_valid() {
                query.p0().iter_mut().for_each(|(_, mut vis, _)| {
                    // `Invisible` option: make all pieces visible when game ends.
                    *vis = Visibility::Inherited;
                });

                play_sound.send(PlaySoundEvent::GameOver);
                game_state.set(GameState::Over);
                player_phase.set(PlayerPhase::Over);
            } else {
                player_data.can_press_down = false; // keep pressing down will not affect next piece

                let min_y = player_data
                    .board
                    .curr_piece_to_squares_with_pos()
                    .iter()
                    .fold(19, |acc, sqr| acc.min(sqr.1 as u64));
                player_data.entry_delay_timer = EntryDelayTimer::new(min_y, game_config.tv_system);

                player_data.board.lock_curr_piece();
                query.p1().iter_mut().for_each(|mut tf| {
                    // make invisible
                    tf.translation.z = transform.board_translation().z - 1.0;
                });

                let lines = player_data.board.get_line_clear_rows();

                query
                    .p0()
                    .iter_mut()
                    .for_each(|(mut sprite, mut vis, coord)| {
                        sprite.image = square_image_assets.get_image(
                            SquareImageSize::Standard,
                            player_data.board.get_square(coord.0 as i32, coord.1 as i32),
                        );
                        if lines.contains(&coord.1) {
                            // `Invisible` option: make squares visible when line clear.
                            *vis = Visibility::Inherited;
                        }
                    });

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
            Query<(&mut Sprite, &BoardSquareEntityMarker)>,
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
                for (mut sprite, coord) in query.p0().iter_mut() {
                    if (coord.0 == left || coord.0 == right)
                        && player_data.line_clear_rows.contains(&coord.1)
                    {
                        sprite.image =
                            square_image_assets.get_image(SquareImageSize::Standard, Piece::X);
                    }
                }
                if player_data.line_clear_rows.len() == 4 {
                    if let Ok(mut vis) = query.p1().get_single_mut() {
                        match *vis {
                            Visibility::Hidden => *vis = Visibility::Inherited,
                            Visibility::Inherited => *vis = Visibility::Hidden,
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
            Query<(&mut Sprite, &mut Visibility, &BoardSquareEntityMarker)>,
            Query<(&mut Transform, &mut Sprite), With<CurrPieceEntityMarker>>,
            Query<(&mut Transform, &mut Sprite, &NextPieceEntityMarker)>,
            Query<(&mut Sprite, &PieceCountEntityMarker)>,
        )>,
        game_config: Res<GameConfig>,
        mut player_data: ResMut<PlayerData>,
        mut player_phase: ResMut<NextState<PlayerPhase>>,
        square_image_assets: Res<SquareImageAssets>,
        transform: Res<GameTransform>,
    ) {
        if player_data.entry_delay_timer.tick(time.delta()).consume() {
            player_data.board.switch_to_next_piece();

            if let Ok(mut vis) = query.p0().get_single_mut() {
                *vis = Visibility::Hidden;
            }

            query
                .p1()
                .iter_mut()
                .for_each(|(mut sprite, mut vis, coord)| {
                    sprite.image = square_image_assets.get_image(
                        SquareImageSize::Standard,
                        player_data.board.get_square(coord.0 as i32, coord.1 as i32),
                    );
                    // `Invisible` option: make all squares follow the visibility option
                    *vis = game_config.invisible.into();
                });

            std::iter::zip(
                query.p2().iter_mut(),
                player_data.board.curr_piece_to_squares_with_pos(),
            )
            .for_each(|((mut tf, mut sprite), sqr)| {
                sprite.image = square_image_assets
                    .get_image(SquareImageSize::Standard, *player_data.board.curr_piece());
                tf.translation = transform.curr_piece_translation(sqr.0, sqr.1);
            });
            std::iter::zip(
                query.p3().iter_mut(),
                player_data
                    .board
                    .next_pieces()
                    .iter()
                    .flat_map(|piece| piece.to_squares_with_piece_center_align())
                    .collect::<Vec<_>>(),
            )
            .for_each(|((mut tf, mut sprite, index), sqr)| {
                sprite.image = square_image_assets.get_image(
                    SquareImageSize::Standard,
                    player_data.board.next_pieces()[index.0],
                );
                tf.translation = transform.next_piece_translation(sqr.0, sqr.1, index.0);
            });
            query.p4().iter_mut().for_each(|(mut sprite, piece)| {
                sprite.image = square_image_assets.get_image(SquareImageSize::Small, piece.0);
            });

            player_phase.set(PlayerPhase::Dropping);
        }
    }
}

mod state_game_pause {
    use super::*;

    pub(super) fn handle_input_system(
        keys: Res<ButtonInput<KeyCode>>,
        gamepads: Query<&Gamepad>,
        controller_mapping: Res<ControllerMapping>,
        mut query: Query<&mut Visibility, With<BoardCoverEntityMarker>>,
        mut play_sound: EventWriter<PlaySoundEvent>,
        mut game_state: ResMut<NextState<GameState>>,
        mut app_state: ResMut<NextState<AppState>>,
    ) {
        let player_inputs = PlayerInputs::with_keyboard(&keys)
            | PlayerInputs::with_gamepads(gamepads, *controller_mapping);

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
        gamepads: Query<&Gamepad>,
        controller_mapping: Res<ControllerMapping>,
        mut play_sound: EventWriter<PlaySoundEvent>,
        mut app_state: ResMut<NextState<AppState>>,
    ) {
        let player_inputs = PlayerInputs::with_keyboard(&keys)
            | PlayerInputs::with_gamepads(gamepads, *controller_mapping);

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
