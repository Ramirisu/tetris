use bevy::{
    color::palettes::css::{BLACK, GREEN, RED, WHITE, YELLOW},
    prelude::*,
};

use crate::{
    app_state::AppState,
    audio::plugin::PlaySoundEvent,
    input::{controller_mapping::ControllerMapping, player_inputs::PlayerInputs},
    settings_menu::scale_factor::{WINDOW_HEIGHT, WINDOW_WIDTH},
    utility::{effect::flicker, entity::despawn_all, format::format_hhmmss},
};

use super::{
    asset::{SquareImageAssets, SquareImageDisplayLevel},
    board::Board,
    game::{GameConfig, GameState},
    invisible::Invisible,
    linecap::Linecap,
    palette::SquareImageSize,
    piece::Piece,
    player::{LineClearPhase, PlayerData, PlayerPhase},
};

pub fn setup(app: &mut App) {
    app.init_state::<GameState>()
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
                        state_player_init::init_system, //
                    )
                        .run_if(in_state(PlayerPhase::Init)),
                    (
                        increase_stopwatch_system,
                        state_player_dropping::handle_input_system,
                        state_player_dropping::drop_curr_piece_system,
                        update_statistics_system,
                    )
                        .chain()
                        .run_if(in_state(PlayerPhase::Dropping)),
                    (
                        increase_stopwatch_system,
                        state_player_line_clear::clear_lines_system,
                        update_statistics_system,
                    )
                        .chain()
                        .run_if(in_state(PlayerPhase::LineClear)),
                    (
                        increase_stopwatch_system,
                        state_player_entry_delay::deploy_new_piece_system,
                        update_statistics_system,
                    )
                        .run_if(in_state(PlayerPhase::EntryDelay)),
                )
                    .run_if(in_state(GameState::Running)),
                (state_game_pause::handle_input_system,) //
                    .run_if(in_state(GameState::Pause)),
                (
                    state_game_over::handle_input_system,
                    update_statistics_system,
                ) //
                    .run_if(in_state(GameState::Over)),
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
struct BackgroundFlickeringEntityMarker;

#[derive(Component)]
struct PauseScreenEntityMarker;

#[derive(Component)]
struct LinesEntityMarker;

#[derive(Component)]
struct ScoreEntityMarker;

#[derive(Component)]
struct LevelEntityMarker;

#[derive(Component)]
struct DASCounterEntityMarker;

#[derive(Component)]
struct DASCounterBarEntityMarker(u64);

#[derive(Component)]
struct GameStopwatchEntityMarker;

#[derive(Debug, Component)]
enum GameStatisticsEntityMarker {
    Burned,
    TetrisRate,
    Drought,
}

const BOARD_SQUARE_SIZE: f32 = 40.0;
const BORDER_WIDTH: f32 = 4.0;

#[derive(Clone, Copy, Component)]
struct PieceDistributionIconEntityMarker(Piece);

impl From<Piece> for PieceDistributionIconEntityMarker {
    fn from(value: Piece) -> Self {
        PieceDistributionIconEntityMarker(value)
    }
}

#[derive(Component)]
struct PieceDistributionTextEntityMarker(Piece);

#[derive(Clone, Copy, Component)]
struct BurnedIconEntityMarker;

#[derive(Clone, Copy, Component)]
struct TetrisRateIconEntityMarker;

#[derive(Clone, Copy, Component)]
struct DroughtIconEntityMarker;

#[derive(Component)]
struct NextPieceEntityMarker {
    pub idx: usize,
    pub x: i32,
    pub y: i32,
    pub scale: f32,
}

impl NextPieceEntityMarker {
    pub fn new(idx: usize, x: i32, y: i32, scale: f32) -> Self {
        Self { idx, x, y, scale }
    }
}

#[derive(Component)]
enum PlayerInputsEntityMarker {
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
    player_data: Res<PlayerData>,
) {
    commands.insert_resource(SquareImageAssets::new(
        &mut image_assets,
        player_data.board.level(),
    ));
}

fn unload_assets(mut commands: Commands) {
    commands.remove_resource::<SquareImageAssets>();
}

fn setup_screen(
    mut commands: Commands,
    game_config: Res<GameConfig>,
    player_data: Res<PlayerData>,
) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.0)),
            BackgroundFlickeringEntityMarker,
            GameEntityMarker,
        ))
        .with_children(|p| {
            p.spawn(Node {
                width: Val::Px(WINDOW_WIDTH),
                height: Val::Px(WINDOW_HEIGHT),
                display: Display::Grid,
                grid_template_columns: vec![GridTrack::auto(); 3],
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            })
            .with_children(|p| {
                const PANEL_PADDING: UiRect = UiRect::axes(Val::Px(10.0), Val::Px(50.0));

                setup_left_panel(
                    p.spawn(Node {
                        width: Val::Px(500.0),
                        height: Val::Px(WINDOW_HEIGHT),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Start,
                        align_items: AlignItems::End,
                        padding: PANEL_PADDING,
                        border: UiRect::all(Val::Px(1.0)), // cfg!(debug_assertions)
                        ..default()
                    })
                    .insert_if(BorderColor::from(WHITE), || cfg!(debug_assertions)),
                );
                setup_central_panel(
                    p.spawn(Node {
                        width: Val::Px(420.0),
                        height: Val::Px(WINDOW_HEIGHT),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: PANEL_PADDING,
                        border: UiRect::all(Val::Px(1.0)), // cfg!(debug_assertions)
                        ..default()
                    })
                    .insert_if(BorderColor::from(WHITE), || cfg!(debug_assertions)),
                    &player_data,
                );
                setup_right_panel(
                    p.spawn(Node {
                        width: Val::Px(500.0),
                        height: Val::Px(WINDOW_HEIGHT),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Start,
                        align_items: AlignItems::Start,
                        padding: PANEL_PADDING,
                        border: UiRect::all(Val::Px(1.0)), // cfg!(debug_assertions)
                        ..default()
                    })
                    .insert_if(BorderColor::from(WHITE), || cfg!(debug_assertions)),
                    &game_config,
                );
            });
        });

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                overflow: Overflow::clip(),
                margin: UiRect::AUTO,
                ..default()
            },
            BackgroundColor::from(BLACK),
            ZIndex(100),
            Visibility::Hidden,
            PauseScreenEntityMarker,
            GameEntityMarker,
        ))
        .with_child((
            Text::new(t!("tetris.game.pause_screen")),
            TextFont::from_font_size(60.0),
            TextColor::from(WHITE),
            TextLayout::new_with_justify(JustifyText::Center),
        ));
}

fn setup_left_panel(p: &mut EntityCommands) {
    p.with_children(|p| {
        // LINES
        p.spawn(Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::End,
            margin: UiRect::all(Val::Px(10.0)),
            ..default()
        })
        .with_children(|p| {
            p.spawn((
                Text::new(t!("tetris.game.lines")),
                TextFont::from_font_size(40.0),
                TextColor::from(WHITE),
                TextLayout::new_with_justify(JustifyText::Right),
            ));
            p.spawn((
                Text::default(),
                TextFont::from_font_size(80.0),
                TextColor::from(WHITE),
                TextLayout::new_with_justify(JustifyText::Right),
                LinesEntityMarker,
            ));
        });

        // PIECE DISTRIBUTION
        p.spawn(Node {
            display: Display::Grid,
            grid_template_columns: vec![GridTrack::auto(); 2],
            column_gap: Val::Px(20.0),
            row_gap: Val::Px(20.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::axes(Val::Px(20.0), Val::Px(50.0)),
            ..default()
        })
        .with_children(|p| {
            Piece::iter()
                .filter(|piece| **piece != Piece::X)
                .for_each(|piece| {
                    spawn_piece_icon(
                        p,
                        *piece,
                        Val::Px(25.0),
                        PieceDistributionIconEntityMarker::from(*piece),
                    );
                    p.spawn((
                        Text::default(),
                        TextFont::from_font_size(40.0),
                        TextColor::from(WHITE),
                        TextLayout::new_with_justify(JustifyText::Left),
                        PieceDistributionTextEntityMarker(*piece),
                    ));
                });
        });

        p.spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: Val::Px(1.0),
                border: UiRect::all(Val::Px(BORDER_WIDTH)),
                ..default()
            },
            BorderColor::from(WHITE),
            BackgroundColor::from(WHITE),
        ))
        .with_children(|p| {
            fn spawn_info_block<Marker: Component + Copy>(
                p: &mut ChildSpawnerCommands,
                piece: Piece,
                text_marker: GameStatisticsEntityMarker,
                icon_marker: Marker,
            ) {
                p.spawn((
                    Node {
                        width: Val::Auto,
                        height: Val::Percent(100.0),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor::from(BLACK),
                ))
                .with_children(|p| {
                    const SQUARE_SIZE: f32 = 20.0;

                    p.spawn(Node {
                        width: Val::Auto,
                        height: Val::Px(SQUARE_SIZE * 2.5),
                        ..default()
                    })
                    .with_children(|p| {
                        spawn_piece_icon(p, piece, Val::Px(SQUARE_SIZE), icon_marker);
                    });

                    p.spawn((
                        Node {
                            margin: UiRect::all(Val::Px(5.0)),
                            ..default()
                        },
                        Text::default(),
                        TextFont::from_font_size(30.0),
                        TextColor::from(WHITE),
                        TextLayout::new_with_justify(JustifyText::Center),
                        text_marker,
                    ));
                });
            }

            // BURNED
            spawn_info_block(
                p,
                Piece::o(),
                GameStatisticsEntityMarker::Burned,
                BurnedIconEntityMarker,
            );

            // TETRIS RATE
            spawn_info_block(
                p,
                Piece::t(),
                GameStatisticsEntityMarker::TetrisRate,
                TetrisRateIconEntityMarker,
            );

            // DROUGHT
            spawn_info_block(
                p,
                Piece::i(),
                GameStatisticsEntityMarker::Drought,
                DroughtIconEntityMarker,
            );
        });

        // TIME
        p.spawn(Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::End,
            margin: UiRect::all(Val::Px(10.0)),
            ..default()
        })
        .with_children(|p| {
            p.spawn((
                Text::new(t!("tetris.game.time")),
                TextFont::from_font_size(20.0),
                TextColor::from(WHITE),
                TextLayout::new_with_justify(JustifyText::Right),
            ));
            p.spawn((
                Text::default(),
                TextFont::from_font_size(30.0),
                TextColor::from(WHITE),
                TextLayout::new_with_justify(JustifyText::Right),
                GameStopwatchEntityMarker,
            ));
        });
    });
}

fn setup_central_panel(p: &mut EntityCommands, player_data: &PlayerData) {
    p.with_children(|p| {
        p.spawn(Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|p| {
            p.spawn(Node {
                display: Display::Grid,
                grid_template_columns: vec![GridTrack::auto(); 3],
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: Val::Px(2.0),
                margin: UiRect::top(Val::Px(50.0)),
                ..default()
            })
            .with_children(|p| {
                // BOARD
                p.spawn((
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor::from(BLACK),
                ))
                .with_children(|p| {
                    fn spawn_row(p: &mut ChildSpawnerCommands, y: usize) {
                        for x in 0..Board::BOARD_COLS {
                            p.spawn((
                                Node {
                                    width: Val::Px(36.0),
                                    height: Val::Px(36.0),
                                    ..default()
                                },
                                ImageNode::default(),
                                BoardSquareEntityMarker(x, y),
                            ));
                        }
                    }

                    p.spawn((
                        Node {
                            display: Display::Grid,
                            grid_template_columns: vec![GridTrack::auto(); Board::BOARD_COLS],
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border: UiRect::horizontal(Val::Px(BORDER_WIDTH)),
                            ..default()
                        },
                        BorderColor::from(BLACK),
                        BackgroundColor::from(BLACK),
                    ))
                    .with_children(|p| {
                        for y in (Board::BOARD_ROWS..Board::INTERNAL_BOARD_ROWS).rev() {
                            spawn_row(p, y);
                        }
                    });

                    p.spawn((
                        Node {
                            display: Display::Grid,
                            grid_template_columns: vec![GridTrack::auto(); Board::BOARD_COLS],
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border: UiRect::px(BORDER_WIDTH, BORDER_WIDTH, 0.0, BORDER_WIDTH),
                            ..default()
                        },
                        BorderColor::from(WHITE),
                        BackgroundColor::from(BLACK),
                    ))
                    .with_children(|p| {
                        for y in (0..Board::BOARD_ROWS).rev() {
                            spawn_row(p, y);
                        }
                    });
                });
            });

            // DAS
            p.spawn(Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: Val::Px(20.0),
                margin: UiRect::all(Val::Px(20.0)),
                ..default()
            })
            .with_children(|p| {
                p.spawn((
                    Text::new("DAS"),
                    TextFont::from_font_size(40.0),
                    TextColor::from(WHITE),
                    TextLayout::new_with_justify(JustifyText::Center),
                ));
                p.spawn(Node {
                    width: Val::Auto,
                    height: Val::Auto,
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(0.0),
                    ..default()
                })
                .with_children(|p| {
                    for idx in 0..player_data.das_timer.get_threshold_ticks() {
                        p.spawn((
                            Node {
                                width: Val::Px(12.0),
                                height: Val::Px(36.0),
                                border: UiRect::all(Val::Px(1.0)),
                                ..default()
                            },
                            BorderColor::from(WHITE),
                            BorderRadius::right(Val::Px(12.0)),
                            DASCounterBarEntityMarker(idx),
                        ));
                    }
                });
                p.spawn((
                    Text::default(),
                    TextFont::from_font_size(40.0),
                    TextColor::from(WHITE),
                    TextLayout::new_with_justify(JustifyText::Center),
                    DASCounterEntityMarker,
                ));
            });
        });
    });
}

fn setup_right_panel(p: &mut EntityCommands, game_config: &GameConfig) {
    p.with_children(|p| {
        // SCORE
        p.spawn(Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Stretch,
            margin: UiRect::all(Val::Px(10.0)),
            ..default()
        })
        .with_children(|p| {
            p.spawn((
                Text::new(t!("tetris.game.score")),
                TextFont::from_font_size(40.0),
                TextColor::from(WHITE),
                TextLayout::new_with_justify(JustifyText::Left),
            ));
            p.spawn((
                Text::default(),
                TextFont::from_font_size(80.0),
                TextColor::from(WHITE),
                TextLayout::new_with_justify(JustifyText::Left),
                ScoreEntityMarker,
            ));
        });

        p.spawn(Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Start,
            margin: UiRect::px(10.0, 10.0, 110.0, 10.0),
            ..default()
        })
        .with_children(|p| {
            p.spawn(Node {
                width: Val::Auto,
                height: Val::Auto,
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            })
            .with_child((
                Text::new(t!("tetris.game.next")),
                TextFont::from_font_size(40.0),
                TextColor::from(WHITE),
                TextLayout::new_with_justify(JustifyText::Left),
            ));
            // NEXT PIECE (0)
            spawn_next_piece(
                p,
                0,
                1.0,
                Visibility::Inherited,
                game_config.next_piece_hint.as_visibility(0),
            );
        });

        // NEXT PIECE (1..)
        p.spawn(Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Center,
            column_gap: Val::Px(10.0),
            margin: UiRect::all(Val::Px(10.0)),
            ..default()
        })
        .with_children(|p| {
            for idx in 1..5 {
                spawn_next_piece(
                    p,
                    idx,
                    0.5,
                    game_config.next_piece_hint.as_visibility(idx),
                    game_config.next_piece_hint.as_visibility(idx),
                );
            }
        });

        // LEVEL
        p.spawn(Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Center,
            column_gap: Val::Px(40.0),
            margin: UiRect::all(Val::Px(10.0)),
            ..default()
        })
        .with_children(|p| {
            p.spawn((
                Text::new(t!("tetris.game.level")),
                TextFont::from_font_size(40.0),
                TextColor::from(WHITE),
                TextLayout::new_with_justify(JustifyText::Center),
            ));
            p.spawn((
                Text::default(),
                TextFont::from_font_size(80.0),
                TextColor::from(WHITE),
                TextLayout::new_with_justify(JustifyText::Center),
                LevelEntityMarker,
            ));
        });

        // PLAYER INPUTS
        p.spawn((
            Node {
                width: Val::Px(300.0),
                height: Val::Auto,
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.0)),
                margin: UiRect::all(Val::Px(10.0)),
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            BorderColor::from(WHITE),
        ))
        .with_children(|p| {
            // ARROW BUTTONS
            p.spawn((
                Node {
                    display: Display::Grid,
                    grid_template_columns: vec![GridTrack::auto(); 3],
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    margin: UiRect::all(Val::Px(10.0)),
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                BorderColor::from(WHITE),
            ))
            .with_children(|p| {
                let buttons = vec![
                    None,
                    Some(PlayerInputsEntityMarker::Up),
                    None,
                    Some(PlayerInputsEntityMarker::Left),
                    None,
                    Some(PlayerInputsEntityMarker::Right),
                    None,
                    Some(PlayerInputsEntityMarker::Down),
                    None,
                ];
                for button in buttons {
                    if let Some(marker) = button {
                        p.spawn((
                            Node {
                                width: Val::Px(20.0),
                                height: Val::Px(20.0),
                                ..default()
                            },
                            BackgroundColor::from(WHITE),
                            marker,
                        ));
                    } else {
                        p.spawn(Node {
                            width: Val::Px(20.0),
                            height: Val::Px(20.0),
                            ..default()
                        });
                    }
                }
            });

            // A & B BUTTONS
            p.spawn(Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::top(Val::Auto),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            })
            .with_children(|p| {
                let buttons = vec![PlayerInputsEntityMarker::B, PlayerInputsEntityMarker::A];

                for button in buttons {
                    p.spawn((
                        Node {
                            width: Val::Auto,
                            height: Val::Auto,
                            border: UiRect::all(Val::Px(1.0)),
                            margin: UiRect::horizontal(Val::Px(5.0)),
                            ..default()
                        },
                        BorderColor::from(WHITE),
                    ))
                    .with_child((
                        Node {
                            width: Val::Px(30.0),
                            height: Val::Px(30.0),
                            margin: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                        BackgroundColor::from(WHITE),
                        BorderRadius::all(Val::Px(15.0)),
                        button,
                    ));
                }
            });
        });
    });
}

fn spawn_piece_icon<Marker>(
    p: &mut ChildSpawnerCommands,
    piece: Piece,
    square_size: Val,
    marker: Marker,
) where
    Marker: Component + Copy,
{
    let sqrs = piece.to_squares();
    let (min_x, max_x, min_y, max_y) =
        sqrs.iter()
            .fold((10, -10, 10, -10), |(min_x, max_x, min_y, max_y), sqr| {
                (
                    min_x.min(sqr.0),
                    max_x.max(sqr.0),
                    min_y.min(sqr.1),
                    max_y.max(sqr.1),
                )
            });
    let cols = (max_x - min_x + 1) as usize;

    p.spawn(Node {
        display: Display::Grid,
        grid_template_columns: vec![GridTrack::auto(); cols],
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect::AUTO,
        ..default()
    })
    .with_children(|p| {
        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                p.spawn((
                    Node {
                        width: square_size,
                        height: square_size,
                        ..default()
                    },
                    ImageNode::default(),
                ))
                .insert_if(marker, || sqrs.iter().any(|sqr| sqr.0 == x && sqr.1 == y));
            }
        }
    });
}

fn spawn_next_piece(
    p: &mut ChildSpawnerCommands,
    idx: usize,
    scale: f32,
    block_vis: Visibility,
    piece_vis: Visibility,
) {
    p.spawn((
        Node {
            display: Display::Grid,
            grid_template_columns: vec![GridTrack::auto(); 4],
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(BORDER_WIDTH * scale)),
            padding: UiRect::all(Val::Px(10.0 * scale)),
            ..default()
        },
        BorderColor::from(WHITE),
        block_vis,
    ))
    .with_children(|p| {
        for y in (-2..2).rev() {
            for x in -2..2 {
                p.spawn((
                    Node {
                        width: Val::Px(BOARD_SQUARE_SIZE * scale),
                        height: Val::Px(BOARD_SQUARE_SIZE * scale),
                        ..default()
                    },
                    ImageNode::default(),
                    piece_vis,
                    NextPieceEntityMarker::new(idx, x, y, scale),
                ));
            }
        }
    });
}

fn increase_stopwatch_system(time: Res<Time>, mut player_data: ResMut<PlayerData>) {
    player_data.stopwatch.tick(time.delta());
}

fn update_statistics_system(
    time: Res<Time>,
    mut query: ParamSet<(
        ParamSet<(
            Query<Entity, With<LinesEntityMarker>>,
            Query<Entity, With<ScoreEntityMarker>>,
            Query<Entity, With<LevelEntityMarker>>,
            Query<(Entity, &GameStatisticsEntityMarker)>,
            Query<(Entity, &PieceDistributionTextEntityMarker)>,
            Query<Entity, With<GameStopwatchEntityMarker>>,
            Query<Entity, With<DASCounterEntityMarker>>,
            Query<(&mut BackgroundColor, &DASCounterBarEntityMarker)>,
        )>,
        Query<&mut ImageNode, With<DroughtIconEntityMarker>>,
        Query<&mut ImageNode, With<BurnedIconEntityMarker>>,
    )>,
    mut tw: TextUiWriter,
    game_config: Res<GameConfig>,
    player_data: Res<PlayerData>,
    square_image_assets: Res<SquareImageAssets>,
) {
    let drought_level = match player_data.board.drought() {
        0..7 => SquareImageDisplayLevel::Info,
        7..14 => SquareImageDisplayLevel::Warn,
        _ => SquareImageDisplayLevel::Error,
    };

    let drought_alpha = match drought_level {
        SquareImageDisplayLevel::Info => 1.0,
        SquareImageDisplayLevel::Warn => flicker(time.elapsed_secs(), 1.0),
        SquareImageDisplayLevel::Error => flicker(time.elapsed_secs(), 0.5),
    };

    if let Ok(entity) = query.p0().p0().single_mut() {
        *tw.text(entity, 0) = format!("{:03}", player_data.board.lines());
    }
    if let Ok(entity) = query.p0().p1().single_mut() {
        *tw.text(entity, 0) = game_config.scoring.format(player_data.board.score());
    }
    if let Ok(entity) = query.p0().p2().single_mut() {
        *tw.text(entity, 0) = game_config
            .leveling
            .format_string(player_data.board.level());
    }
    for (entity, marker) in query.p0().p3() {
        match marker {
            GameStatisticsEntityMarker::Burned => {
                *tw.text(entity, 0) = format!("{}", player_data.board.burned_lines())
            }
            GameStatisticsEntityMarker::TetrisRate => {
                if let Some(rate) = player_data.board.clear_lines_rate(4).1 {
                    let rate = (rate * 100.0).round() as usize;
                    *tw.text(entity, 0) = format!("{:3}%", rate);
                    match rate {
                        0..50 => *tw.color(entity, 0) = RED.into(),
                        50..80 => *tw.color(entity, 0) = YELLOW.into(),
                        _ => *tw.color(entity, 0) = GREEN.into(),
                    }
                } else {
                    *tw.text(entity, 0) = format!("-%");
                    *tw.color(entity, 0) = WHITE.into()
                }
            }
            GameStatisticsEntityMarker::Drought => {
                *tw.text(entity, 0) = format!("{}", player_data.board.drought());
                *tw.color(entity, 0) = {
                    let mut color = drought_level.color();
                    color.set_alpha(drought_alpha);
                    color
                }
                .into();
            }
        }
    }
    for (entity, piece) in query.p0().p4() {
        *tw.text(entity, 0) = format!("{:03}", player_data.board.get_piece_count(piece.0));
    }
    if let Ok(entity) = query.p0().p5().single_mut() {
        *tw.text(entity, 0) = format_hhmmss(player_data.stopwatch.elapsed());
    }

    let das_color = if player_data.das_timer.is_active() {
        GREEN
    } else {
        RED
    };
    let das_ticks = game_config
        .tv_system
        .duration_to_ticks(player_data.das_timer.elapsed());
    if let Ok(entity) = query.p0().p6().single_mut() {
        *tw.text(entity, 0) = format!("{:02}", das_ticks);
        *tw.color(entity, 0) = das_color.into();
    }
    for (mut bg_color, marker) in query.p0().p7() {
        if marker.0 < das_ticks {
            *bg_color = das_color.into();
        } else {
            *bg_color = BLACK.into();
        }
    }

    for mut img in query.p1() {
        img.image = square_image_assets.get_display_level_image(drought_level);
        img.color.set_alpha(drought_alpha);
    }
    for mut img in query.p2() {
        img.image = square_image_assets.get_burned_image();
    }
}

fn update_board(
    query: Query<(&mut ImageNode, &BoardSquareEntityMarker)>,
    player_data: &PlayerData,
    game_config: &GameConfig,
    square_image_assets: &SquareImageAssets,
    force_all_visible: bool,
    force_line_visible: Option<&Vec<usize>>,
) {
    let curr_piece_pos = player_data.board.curr_piece_to_squares_with_pos();
    for (mut img, marker) in query {
        if curr_piece_pos
            .iter()
            .any(|sqr| sqr.0 == marker.0 as i32 && sqr.1 == marker.1 as i32)
        {
            img.image = square_image_assets
                .get_image(SquareImageSize::Standard, *player_data.board.curr_piece());
        } else if force_all_visible
            || game_config.invisible == Invisible::Off
            || force_line_visible
                .map(|lines| lines.contains(&marker.1))
                .unwrap_or(false)
        {
            img.image = square_image_assets.get_image(
                SquareImageSize::Standard,
                player_data
                    .board
                    .get_square(marker.0 as i32, marker.1 as i32),
            );
        } else {
            img.image = square_image_assets.get_image(SquareImageSize::Standard, Piece::X);
        }
    }
}

fn update_next_piece_icons(
    query: Query<(
        &mut Node,
        &mut ImageNode,
        &mut Visibility,
        &NextPieceEntityMarker,
    )>,
    player_data: &PlayerData,
    square_image_assets: &SquareImageAssets,
) {
    for (mut node, mut img, mut vis, marker) in query {
        let piece = player_data.board.next_pieces()[marker.idx];

        let shift: (f32, f32) = match piece {
            Piece::T(_) => (-0.5, 0.0),
            Piece::J(_) => (-0.5, 0.0),
            Piece::Z(_) => (-0.5, 0.0),
            Piece::O(_) => (0.0, 0.0),
            Piece::S(_) => (-0.5, 0.0),
            Piece::L(_) => (-0.5, 0.0),
            Piece::I(_) => (0.0, 0.5),
            Piece::X => (0.0, 0.0),
        };

        node.left = Val::Px(shift.0 * marker.scale * BOARD_SQUARE_SIZE);
        node.top = Val::Px(shift.1 * marker.scale * BOARD_SQUARE_SIZE);

        if piece
            .to_squares()
            .iter()
            .any(|sqr| sqr.0 == marker.x && sqr.1 == marker.y)
        {
            *vis = Visibility::Inherited;
            img.image = square_image_assets.get_image(SquareImageSize::Standard, piece);
        } else {
            *vis = Visibility::Hidden;
            img.image = square_image_assets.get_image(SquareImageSize::Standard, Piece::X);
        }
    }
}

fn update_piece_distribution_icons(
    query: Query<(&mut ImageNode, &PieceDistributionIconEntityMarker)>,
    square_image_assets: &SquareImageAssets,
) {
    for (mut img, marker) in query {
        img.image = square_image_assets.get_image(SquareImageSize::Small, marker.0);
    }
}

fn update_tetris_rate_icon(
    query: Query<&mut ImageNode, With<TetrisRateIconEntityMarker>>,
    square_image_assets: &SquareImageAssets,
) {
    for mut img in query {
        img.image = square_image_assets.get_image(SquareImageSize::Small, Piece::t());
    }
}

fn handle_game_over(
    play_sound: &mut EventWriter<PlaySoundEvent>,
    game_state: &mut NextState<GameState>,
    player_phase: &mut NextState<PlayerPhase>,
) {
    play_sound.write(PlaySoundEvent::GameOver);
    game_state.set(GameState::Over);
    player_phase.set(PlayerPhase::Over);
}

mod state_player_init {
    use super::*;

    pub(super) fn init_system(
        mut query: ParamSet<(
            Query<(&mut ImageNode, &BoardSquareEntityMarker)>,
            Query<(
                &mut Node,
                &mut ImageNode,
                &mut Visibility,
                &NextPieceEntityMarker,
            )>,
            Query<(&mut ImageNode, &PieceDistributionIconEntityMarker)>,
            Query<&mut ImageNode, With<TetrisRateIconEntityMarker>>,
        )>,
        player_data: Res<PlayerData>,
        game_config: Res<GameConfig>,
        square_image_assets: Res<SquareImageAssets>,
        mut player_phase: ResMut<NextState<PlayerPhase>>,
    ) {
        update_board(
            query.p0(),
            &player_data,
            &game_config,
            &square_image_assets,
            false,
            None,
        );
        update_next_piece_icons(query.p1(), &player_data, &square_image_assets);
        update_piece_distribution_icons(query.p2(), &square_image_assets);
        update_tetris_rate_icon(query.p3(), &square_image_assets);
        player_phase.set(PlayerPhase::Dropping);
    }
}

mod state_player_dropping {
    use crate::game::timer::EntryDelayTimer;

    use super::*;

    pub(super) fn handle_input_system(
        time: Res<Time>,
        keys: Res<ButtonInput<KeyCode>>,
        gamepads: Query<&Gamepad>,
        controller_mapping: Res<ControllerMapping>,
        mut query: ParamSet<(
            Query<(&mut ImageNode, &BoardSquareEntityMarker)>,
            Query<&mut Visibility, With<PauseScreenEntityMarker>>,
            Query<(&mut BackgroundColor, &PlayerInputsEntityMarker)>,
        )>,
        mut play_sound: EventWriter<PlaySoundEvent>,
        mut player_data: ResMut<PlayerData>,
        game_config: Res<GameConfig>,
        mut game_state: ResMut<NextState<GameState>>,
        mut app_state: ResMut<NextState<AppState>>,
        square_image_assets: Res<SquareImageAssets>,
    ) {
        let player_inputs = PlayerInputs::with_keyboard(&keys)
            | PlayerInputs::with_gamepads(gamepads, *controller_mapping);

        if player_inputs.soft_reset {
            play_sound.write(PlaySoundEvent::StartGame);
            app_state.set(AppState::SplashScreen);
            return;
        }

        if player_inputs.start.just_pressed {
            if let Ok(mut vis) = query.p1().single_mut() {
                *vis = Visibility::Visible;
            }
            game_state.set(GameState::Pause);
            return;
        }

        for (mut bg_color, marker) in query.p2() {
            let pressed = match marker {
                PlayerInputsEntityMarker::Left => player_inputs.left.pressed,
                PlayerInputsEntityMarker::Right => player_inputs.right.pressed,
                PlayerInputsEntityMarker::Up => player_inputs.up.pressed,
                PlayerInputsEntityMarker::Down => player_inputs.down.pressed,
                PlayerInputsEntityMarker::A => player_inputs.a.pressed,
                PlayerInputsEntityMarker::B => player_inputs.b.pressed,
            };
            if pressed {
                *bg_color = RED.into();
            } else {
                *bg_color = WHITE.into();
            }
        }

        player_data.soft_drop_timer.tick(time.delta());

        let (moved, horizontally_moved, rotated) =
            handle_input(&player_inputs, &time, &mut player_data);
        if moved {
            update_board(
                query.p0(),
                &player_data,
                &game_config,
                &square_image_assets,
                false,
                None,
            );
        }
        if horizontally_moved {
            play_sound.write(PlaySoundEvent::MoveCurrPiece);
        }
        if rotated {
            play_sound.write(PlaySoundEvent::RotateCurrPiece);
        }
    }

    fn handle_input(
        inputs: &PlayerInputs,
        time: &Time,
        player_data: &mut PlayerData,
    ) -> (bool, bool, bool) {
        let mut down_moved = false;
        let mut horizontally_moved = false;
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
            player_data
                .soft_drop_timer
                .set_level(player_data.board.level());
            player_data.press_down_timer.reset();
        }

        if !inputs.down.pressed {
            player_data.press_down_timer.reset();

            if inputs.left.just_pressed || inputs.right.just_pressed {
                player_data.das_timer.reset();
                match (inputs.left.just_pressed, inputs.right.just_pressed) {
                    (true, false) => horizontally_moved |= player_data.board.move_piece_left(),
                    (false, true) => horizontally_moved |= player_data.board.move_piece_right(),
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
                            horizontally_moved |= player_data.board.move_piece_left();
                        }
                    }
                    (false, true) => {
                        if !player_data.board.is_right_movable() {
                            player_data.das_timer.charge();
                        } else if player_data.das_timer.tick(time.delta()).consume() {
                            horizontally_moved |= player_data.board.move_piece_right();
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

        (
            down_moved | horizontally_moved | rotated,
            horizontally_moved,
            rotated,
        )
    }

    pub(super) fn drop_curr_piece_system(
        query: Query<(&mut ImageNode, &BoardSquareEntityMarker)>,
        mut play_sound: EventWriter<PlaySoundEvent>,
        mut game_state: ResMut<NextState<GameState>>,
        mut player_phase: ResMut<NextState<PlayerPhase>>,
        game_config: Res<GameConfig>,
        mut player_data: ResMut<PlayerData>,
        square_image_assets: Res<SquareImageAssets>,
    ) {
        let lock_curr_piece = {
            if std::mem::replace(&mut player_data.lock_curr_piece_immediately, false) {
                player_data.soft_drop_timer.reset();
                true
            } else {
                player_data.soft_drop_timer.consume()
            }
        };

        if lock_curr_piece {
            let new_level = player_data.board.level();
            player_data.soft_drop_timer.set_level(new_level);

            if player_data.board.move_piece_down() {
                update_board(
                    query,
                    &player_data,
                    &game_config,
                    &square_image_assets,
                    false,
                    None,
                );
            } else if !player_data.board.is_curr_position_valid() {
                update_board(
                    query,
                    &player_data,
                    &game_config,
                    &square_image_assets,
                    true,
                    None,
                );

                handle_game_over(&mut play_sound, &mut game_state, &mut player_phase);
            } else {
                player_data.can_press_down = false; // keep pressing down will not affect next piece

                let min_y = player_data
                    .board
                    .curr_piece_to_squares_with_pos()
                    .iter()
                    .fold(19, |acc, sqr| acc.min(sqr.1 as u64));
                player_data.entry_delay_timer = EntryDelayTimer::new(min_y, game_config.tv_system);

                player_data.board.lock_curr_piece();
                let lines = player_data.board.get_line_clear_rows();

                update_board(
                    query,
                    &player_data,
                    &game_config,
                    &square_image_assets,
                    false,
                    Some(&lines),
                );

                match lines.len() {
                    0 => play_sound.write(PlaySoundEvent::LockCurrPiece),
                    1 | 2 | 3 => play_sound.write(PlaySoundEvent::LineClear),
                    4 => play_sound.write(PlaySoundEvent::TetrisClear),
                    _ => unreachable!(),
                };

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

    pub(super) fn clear_lines_system(
        time: Res<Time>,
        mut query: ParamSet<(
            Query<(&mut ImageNode, &BoardSquareEntityMarker)>,
            Query<&mut BackgroundColor, With<BackgroundFlickeringEntityMarker>>,
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
                for (mut img, coord) in query.p0() {
                    if (coord.0 == left || coord.0 == right)
                        && player_data.line_clear_rows.contains(&coord.1)
                    {
                        img.image =
                            square_image_assets.get_image(SquareImageSize::Standard, Piece::X);
                    }
                }
                if player_data.line_clear_rows.len() == 4 {
                    if let Ok(mut bg_color) = query.p1().single_mut() {
                        match bg_color.0.alpha() {
                            0.0 => bg_color.0.set_alpha(1.0),
                            1.0 => bg_color.0.set_alpha(0.0),
                            _ => unreachable!(),
                        }
                    }
                }
            }

            if to_next_state {
                let (new_level, old_level) = player_data.board.clear_lines();
                if new_level > old_level {
                    play_sound.write(PlaySoundEvent::LevelUp);
                    player_data.soft_drop_timer.set_level(new_level);
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

    pub(super) fn deploy_new_piece_system(
        time: Res<Time>,
        mut query: ParamSet<(
            Query<&mut BackgroundColor, With<BackgroundFlickeringEntityMarker>>,
            Query<(&mut ImageNode, &BoardSquareEntityMarker)>,
            Query<(
                &mut Node,
                &mut ImageNode,
                &mut Visibility,
                &NextPieceEntityMarker,
            )>,
            Query<(&mut ImageNode, &PieceDistributionIconEntityMarker)>,
            Query<&mut ImageNode, With<TetrisRateIconEntityMarker>>,
        )>,
        game_config: Res<GameConfig>,
        mut player_data: ResMut<PlayerData>,
        mut player_phase: ResMut<NextState<PlayerPhase>>,
        mut play_sound: EventWriter<PlaySoundEvent>,
        mut game_state: ResMut<NextState<GameState>>,
        square_image_assets: Res<SquareImageAssets>,
    ) {
        if player_data.entry_delay_timer.tick(time.delta()).consume() {
            player_data.board.switch_to_next_piece();

            if let Ok(mut bg_color) = query.p0().single_mut() {
                bg_color.0.set_alpha(0.0);
            }

            update_board(
                query.p1(),
                &player_data,
                &game_config,
                &square_image_assets,
                false,
                None,
            );
            update_next_piece_icons(query.p2(), &player_data, &square_image_assets);
            update_piece_distribution_icons(query.p3(), &square_image_assets);
            update_tetris_rate_icon(query.p4(), &square_image_assets);

            if game_config.linecap == Linecap::Halt
                && player_data.board.level() >= game_config.linecap_level
            {
                handle_game_over(&mut play_sound, &mut game_state, &mut player_phase);
            } else {
                player_phase.set(PlayerPhase::Dropping);
            }
        }
    }
}

mod state_game_pause {
    use super::*;

    pub(super) fn handle_input_system(
        keys: Res<ButtonInput<KeyCode>>,
        gamepads: Query<&Gamepad>,
        controller_mapping: Res<ControllerMapping>,
        mut query: Query<&mut Visibility, With<PauseScreenEntityMarker>>,
        mut play_sound: EventWriter<PlaySoundEvent>,
        mut game_state: ResMut<NextState<GameState>>,
        mut app_state: ResMut<NextState<AppState>>,
    ) {
        let player_inputs = PlayerInputs::with_keyboard(&keys)
            | PlayerInputs::with_gamepads(gamepads, *controller_mapping);

        if player_inputs.soft_reset {
            play_sound.write(PlaySoundEvent::StartGame);
            app_state.set(AppState::SplashScreen);
            return;
        }

        if player_inputs.start.just_pressed {
            if let Ok(mut vis) = query.single_mut() {
                *vis = Visibility::Hidden;
            }
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
            play_sound.write(PlaySoundEvent::StartGame);
            app_state.set(AppState::SplashScreen);
            return;
        }

        if player_inputs.start.just_pressed {
            app_state.set(AppState::LevelMenu);
        }
    }
}
