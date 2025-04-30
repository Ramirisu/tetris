use bevy::{
    color::palettes::css::{BLACK, GREEN, RED, WHITE, YELLOW},
    ecs::relationship::RelatedSpawnerCommands,
    prelude::*,
};

use crate::{
    app_state::AppState,
    audio::plugin::PlaySoundEvent,
    input::{controller_mapping::ControllerMapping, player_inputs::PlayerInputs},
    utility::{despawn_all, format_hhmmss},
};

use super::{
    asset::SquareImageAssets,
    board::Board,
    game::{GameConfig, GameState},
    invisible::Invisible,
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
                    (state_player_init::init_system,).run_if(in_state(PlayerPhase::Init)),
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
struct GameModeEntityMarker(usize);

#[derive(Component)]
struct GameStopwatchEntityMarker;

#[derive(Component)]
struct GameStatisticsEntityMarker(usize);

#[derive(Component)]
struct PieceStatisticsEntityMarker {
    pub piece: Piece,
    pub x: i32,
    pub y: i32,
}

impl PieceStatisticsEntityMarker {
    pub fn new(piece: Piece, x: i32, y: i32) -> Self {
        Self { piece, x, y }
    }
}

#[derive(Component)]
struct PieceStatisticsCounterEntityMarker(Piece);

#[derive(Component)]
struct NextPieceEntityMarker {
    pub idx: usize,
    pub x: i32,
    pub y: i32,
}

impl NextPieceEntityMarker {
    pub fn new(idx: usize, x: i32, y: i32) -> Self {
        Self { idx, x, y }
    }
}

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
    square_image_assets: Res<SquareImageAssets>,
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
        .with_children(|parent| {
            parent
                .spawn(Node {
                    display: Display::Grid,
                    grid_template_columns: vec![
                        GridTrack::px(450.0),
                        GridTrack::auto(),
                        GridTrack::px(450.0),
                    ],
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Start,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                })
                .with_children(|parent| {
                    // left
                    parent
                        .spawn(Node {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Start,
                            align_items: AlignItems::End,
                            ..default()
                        })
                        .with_children(|parent| {
                            // LINES
                            parent
                                .spawn((
                                    Node {
                                        width: Val::Px(300.0),
                                        height: Val::Auto,
                                        display: Display::Flex,
                                        flex_direction: FlexDirection::Column,
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        border: UiRect::all(Val::Px(1.0)),
                                        margin: UiRect::all(Val::Px(10.0)),
                                        padding: UiRect::all(Val::Px(10.0)),
                                        ..default()
                                    },
                                    BorderColor::from(WHITE),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        Text::new("LINES"),
                                        TextFont::from_font_size(40.0),
                                        TextColor::from(WHITE),
                                        TextLayout::new_with_justify(JustifyText::Center),
                                    ));
                                    parent.spawn((
                                        Text::default(),
                                        TextFont::from_font_size(60.0),
                                        TextColor::from(WHITE),
                                        TextLayout::new_with_justify(JustifyText::Center),
                                        LinesEntityMarker,
                                    ));
                                });

                            // GAME STATISTICS
                            parent
                                .spawn((
                                    Node {
                                        width: Val::Px(300.0),
                                        height: Val::Auto,
                                        display: Display::Grid,
                                        grid_template_columns: vec![GridTrack::auto(); 2],
                                        column_gap: Val::Px(10.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        border: UiRect::all(Val::Px(1.0)),
                                        margin: UiRect::all(Val::Px(10.0)),
                                        padding: UiRect::all(Val::Px(10.0)),
                                        ..default()
                                    },
                                    BorderColor::from(WHITE),
                                ))
                                .with_children(|parent| {
                                    let titles =
                                        ["BRN", "1X", "2X", "3X", "TRT", "TRT", "DRT", "MAX DRT"];
                                    for (idx, title) in titles.iter().enumerate() {
                                        parent.spawn((
                                            Text::new(*title),
                                            TextFont::from_font_size(30.0),
                                            TextColor::from(WHITE),
                                            TextLayout::new_with_justify(JustifyText::Right),
                                        ));
                                        parent.spawn((
                                            Text::default(),
                                            TextFont::from_font_size(30.0),
                                            TextColor::from(WHITE),
                                            TextLayout::new_with_justify(JustifyText::Left),
                                            GameStatisticsEntityMarker(idx),
                                        ));
                                    }
                                });

                            // PIECE STATISTICS
                            parent
                                .spawn((
                                    Node {
                                        width: Val::Px(300.0),
                                        height: Val::Auto,
                                        display: Display::Grid,
                                        grid_template_columns: vec![GridTrack::auto(); 2],
                                        column_gap: Val::Px(10.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        border: UiRect::all(Val::Px(1.0)),
                                        margin: UiRect::all(Val::Px(10.0)),
                                        padding: UiRect::all(Val::Px(10.0)),
                                        ..default()
                                    },
                                    BorderColor::from(WHITE),
                                ))
                                .with_children(|parent| {
                                    Piece::iter().filter(|piece| **piece != Piece::X).for_each(
                                        |piece| {
                                            parent
                                                .spawn(Node {
                                                    display: Display::Grid,
                                                    grid_template_columns: vec![
                                                        GridTrack::auto();
                                                        4
                                                    ],
                                                    justify_content: JustifyContent::Center,
                                                    align_items: AlignItems::Center,
                                                    margin: UiRect::all(Val::Px(10.0)),
                                                    ..default()
                                                })
                                                .with_children(|parent| {
                                                    for y in (-1..1).rev() {
                                                        for x in -2..2 {
                                                            parent.spawn((
                                                                Node {
                                                                    width: Val::Px(20.0),
                                                                    height: Val::Px(20.0),
                                                                    ..default()
                                                                },
                                                                ImageNode {
                                                                    image: square_image_assets
                                                                        .get_image(
                                                                        SquareImageSize::Standard,
                                                                        Piece::X,
                                                                    ),
                                                                    ..default()
                                                                },
                                                                PieceStatisticsEntityMarker::new(
                                                                    *piece, x, y,
                                                                ),
                                                            ));
                                                        }
                                                    }
                                                });
                                            parent.spawn((
                                                Text::default(),
                                                TextFont::from_font_size(30.0),
                                                TextColor::from(WHITE),
                                                TextLayout::new_with_justify(JustifyText::Left),
                                                PieceStatisticsCounterEntityMarker(*piece),
                                            ));
                                        },
                                    );
                                });
                        });

                    // center
                    parent
                        .spawn(Node {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::all(Val::Px(20.0)),
                            ..default()
                        })
                        .with_children(|parent| {
                            // BOARD
                            parent
                                .spawn((
                                    Node {
                                        width: Val::Px(420.0),
                                        height: Val::Auto,
                                        display: Display::Flex,
                                        flex_direction: FlexDirection::Column,
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        margin: UiRect::all(Val::Px(10.0)),
                                        padding: UiRect::all(Val::Px(10.0)),
                                        ..default()
                                    },
                                    BackgroundColor::from(BLACK),
                                ))
                                .with_children(|parent| {
                                    let spawn_row =
                                        |parent: &mut RelatedSpawnerCommands<'_, ChildOf>,
                                         y: usize| {
                                            for x in 0..Board::BOARD_COLS {
                                                parent.spawn((
                                                    Node {
                                                        width: Val::Px(40.0),
                                                        height: Val::Px(40.0),
                                                        ..default()
                                                    },
                                                    ImageNode {
                                                        image: square_image_assets.get_image(
                                                            SquareImageSize::Standard,
                                                            Piece::X,
                                                        ),
                                                        ..default()
                                                    },
                                                    BoardSquareEntityMarker(x, y),
                                                ));
                                            }
                                        };

                                    parent
                                        .spawn((
                                            Node {
                                                width: Val::Auto,
                                                height: Val::Auto,
                                                display: Display::Grid,
                                                grid_template_columns: vec![
                                                    GridTrack::auto();
                                                    Board::BOARD_COLS
                                                ],
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                border: UiRect::horizontal(Val::Px(1.0)),
                                                ..default()
                                            },
                                            BorderColor::from(BLACK),
                                            BackgroundColor::from(BLACK),
                                        ))
                                        .with_children(|parent| {
                                            for y in (Board::BOARD_ROWS..Board::INTERNAL_BOARD_ROWS)
                                                .rev()
                                            {
                                                spawn_row(parent, y);
                                            }
                                        });

                                    parent
                                        .spawn((
                                            Node {
                                                width: Val::Auto,
                                                height: Val::Auto,
                                                display: Display::Grid,
                                                grid_template_columns: vec![
                                                    GridTrack::auto();
                                                    Board::BOARD_COLS
                                                ],
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                border: UiRect {
                                                    left: Val::Px(1.0),
                                                    right: Val::Px(1.0),
                                                    top: Val::Px(0.0),
                                                    bottom: Val::Px(1.0),
                                                },
                                                ..default()
                                            },
                                            BorderColor::from(WHITE),
                                            BackgroundColor::from(BLACK),
                                        ))
                                        .with_children(|parent| {
                                            for y in (0..Board::BOARD_ROWS).rev() {
                                                spawn_row(parent, y);
                                            }
                                        });
                                });

                            // DAS
                            parent
                                .spawn(Node {
                                    width: Val::Auto,
                                    height: Val::Auto,
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Row,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    column_gap: Val::Px(20.0),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn((
                                        Text::new("DAS"),
                                        TextFont::from_font_size(40.0),
                                        TextColor::from(WHITE),
                                        TextLayout::new_with_justify(JustifyText::Center),
                                    ));
                                    parent
                                        .spawn(Node {
                                            width: Val::Auto,
                                            height: Val::Auto,
                                            display: Display::Flex,
                                            flex_direction: FlexDirection::Row,
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            column_gap: Val::Px(0.0),
                                            ..default()
                                        })
                                        .with_children(|parent| {
                                            for idx in
                                                0..player_data.das_timer.get_threshold_ticks()
                                            {
                                                parent.spawn((
                                                    Node {
                                                        width: Val::Px(12.0),
                                                        height: Val::Px(30.0),
                                                        border: UiRect::all(Val::Px(1.0)),
                                                        ..default()
                                                    },
                                                    BorderColor::from(WHITE),
                                                    DASCounterBarEntityMarker(idx),
                                                ));
                                            }
                                        });
                                    parent.spawn((
                                        Text::default(),
                                        TextFont::from_font_size(40.0),
                                        TextColor::from(WHITE),
                                        TextLayout::new_with_justify(JustifyText::Center),
                                        DASCounterEntityMarker,
                                    ));
                                });
                        });

                    let spawn_next_piece =
                        |parent: &mut RelatedSpawnerCommands<'_, ChildOf>,
                         idx: usize,
                         scale: f32,
                         block_vis: Visibility,
                         piece_vis: Visibility| {
                            parent
                                .spawn((
                                    Node {
                                        display: Display::Grid,
                                        grid_template_columns: vec![GridTrack::auto(); 4],
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        border: UiRect::all(Val::Px(1.0)),
                                        padding: UiRect::all(Val::Px(10.0 * scale)),
                                        ..default()
                                    },
                                    BorderColor::from(WHITE),
                                    block_vis,
                                ))
                                .with_children(|parent| {
                                    for y in (-2..2).rev() {
                                        for x in -2..2 {
                                            parent.spawn((
                                                Node {
                                                    width: Val::Px(40.0 * scale),
                                                    height: Val::Px(40.0 * scale),
                                                    ..default()
                                                },
                                                ImageNode {
                                                    image: square_image_assets.get_image(
                                                        SquareImageSize::Standard,
                                                        Piece::X,
                                                    ),
                                                    ..default()
                                                },
                                                piece_vis,
                                                NextPieceEntityMarker::new(idx, x, y),
                                            ));
                                        }
                                    }
                                });
                        };

                    // right
                    parent
                        .spawn(Node {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Start,
                            align_items: AlignItems::Start,
                            ..default()
                        })
                        .with_children(|parent| {
                            // SCORE
                            parent
                                .spawn((
                                    Node {
                                        width: Val::Px(300.0),
                                        height: Val::Auto,
                                        display: Display::Flex,
                                        flex_direction: FlexDirection::Column,
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        border: UiRect::all(Val::Px(1.0)),
                                        margin: UiRect::all(Val::Px(10.0)),
                                        padding: UiRect::all(Val::Px(10.0)),
                                        ..default()
                                    },
                                    BorderColor::from(WHITE),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        Text::new("SCORE"),
                                        TextFont::from_font_size(40.0),
                                        TextColor::from(WHITE),
                                        TextLayout::new_with_justify(JustifyText::Center),
                                    ));
                                    parent.spawn((
                                        Text::default(),
                                        TextFont::from_font_size(60.0),
                                        TextColor::from(WHITE),
                                        TextLayout::new_with_justify(JustifyText::Center),
                                        ScoreEntityMarker,
                                    ));
                                });

                            parent
                                .spawn(Node {
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Row,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::End,
                                    column_gap: Val::Px(20.0),
                                    margin: UiRect::all(Val::Px(10.0)),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent
                                        .spawn(Node {
                                            display: Display::Flex,
                                            flex_direction: FlexDirection::Column,
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        })
                                        .with_children(|parent| {
                                            parent.spawn((
                                                Text::new("NEXT"),
                                                TextFont::from_font_size(40.0),
                                                TextColor::from(WHITE),
                                                TextLayout::new_with_justify(JustifyText::Center),
                                            ));
                                            // NEXT PIECE (0)
                                            spawn_next_piece(
                                                parent,
                                                0,
                                                1.0,
                                                Visibility::Inherited,
                                                game_config.next_piece_hint.as_visibility(0),
                                            );
                                        });

                                    // GAME MODE
                                    parent
                                        .spawn((
                                            Node {
                                                width: Val::Auto,
                                                height: Val::Auto,
                                                display: Display::Grid,
                                                grid_template_columns: vec![GridTrack::auto(); 2],
                                                column_gap: Val::Px(10.0),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                border: UiRect::all(Val::Px(1.0)),
                                                padding: UiRect::all(Val::Px(10.0)),
                                                ..default()
                                            },
                                            BorderColor::from(WHITE),
                                        ))
                                        .with_children(|parent| {
                                            let titles = [
                                                "SLV", "CAP", "TRS", "GRV", "TVS", "INV", "SDG",
                                                "SED",
                                            ];
                                            for (idx, title) in titles.iter().enumerate() {
                                                parent.spawn((
                                                    Text::new(*title),
                                                    TextFont::from_font_size(30.0),
                                                    TextColor::from(WHITE),
                                                    TextLayout::new_with_justify(
                                                        JustifyText::Right,
                                                    ),
                                                ));
                                                parent.spawn((
                                                    Text::default(),
                                                    TextFont::from_font_size(30.0),
                                                    TextColor::from(WHITE),
                                                    TextLayout::new_with_justify(JustifyText::Left),
                                                    GameModeEntityMarker(idx),
                                                ));
                                            }
                                        });
                                });

                            // NEXT PIECE (1..)
                            parent
                                .spawn(Node {
                                    width: Val::Px(300.0),
                                    height: Val::Auto,
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Row,
                                    justify_content: JustifyContent::Start,
                                    align_items: AlignItems::Center,
                                    column_gap: Val::Px(10.0),
                                    margin: UiRect::all(Val::Px(10.0)),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    for idx in 1..5 {
                                        spawn_next_piece(
                                            parent,
                                            idx,
                                            0.5,
                                            game_config.next_piece_hint.as_visibility(idx),
                                            game_config.next_piece_hint.as_visibility(idx),
                                        );
                                    }
                                });

                            // LEVEL
                            parent
                                .spawn((
                                    Node {
                                        width: Val::Px(300.0),
                                        height: Val::Auto,
                                        display: Display::Flex,
                                        flex_direction: FlexDirection::Row,
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        column_gap: Val::Px(20.0),
                                        border: UiRect::all(Val::Px(1.0)),
                                        margin: UiRect::all(Val::Px(10.0)),
                                        padding: UiRect::all(Val::Px(10.0)),
                                        ..default()
                                    },
                                    BorderColor::from(WHITE),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        Text::new("LEVEL"),
                                        TextFont::from_font_size(40.0),
                                        TextColor::from(WHITE),
                                        TextLayout::new_with_justify(JustifyText::Center),
                                    ));
                                    parent.spawn((
                                        Text::default(),
                                        TextFont::from_font_size(60.0),
                                        TextColor::from(WHITE),
                                        TextLayout::new_with_justify(JustifyText::Center),
                                        LevelEntityMarker,
                                    ));
                                });

                            // TIME
                            parent
                                .spawn((
                                    Node {
                                        width: Val::Px(300.0),
                                        height: Val::Auto,
                                        display: Display::Flex,
                                        flex_direction: FlexDirection::Column,
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        border: UiRect::all(Val::Px(1.0)),
                                        margin: UiRect::all(Val::Px(10.0)),
                                        padding: UiRect::all(Val::Px(10.0)),
                                        ..default()
                                    },
                                    BorderColor::from(WHITE),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        Text::default(),
                                        TextFont::from_font_size(30.0),
                                        TextColor::from(WHITE),
                                        TextLayout::new_with_justify(JustifyText::Center),
                                        GameStopwatchEntityMarker,
                                    ));
                                });

                            // PLAYER INPUTS
                            parent
                                .spawn((
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
                                .with_children(|parent| {
                                    // ARROW BUTTONS
                                    parent
                                        .spawn((
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
                                        .with_children(|parent| {
                                            let buttons = vec![
                                                None,
                                                Some(PlayerInputsDisplayEntityMarker::Up),
                                                None,
                                                Some(PlayerInputsDisplayEntityMarker::Left),
                                                None,
                                                Some(PlayerInputsDisplayEntityMarker::Right),
                                                None,
                                                Some(PlayerInputsDisplayEntityMarker::Down),
                                                None,
                                            ];
                                            for button in buttons {
                                                if let Some(marker) = button {
                                                    parent.spawn((
                                                        Node {
                                                            width: Val::Px(20.0),
                                                            height: Val::Px(20.0),
                                                            ..default()
                                                        },
                                                        BackgroundColor::from(WHITE),
                                                        marker,
                                                    ));
                                                } else {
                                                    parent.spawn(Node {
                                                        width: Val::Px(20.0),
                                                        height: Val::Px(20.0),
                                                        ..default()
                                                    });
                                                }
                                            }
                                        });

                                    // A & B BUTTONS
                                    parent
                                        .spawn(Node {
                                            display: Display::Flex,
                                            flex_direction: FlexDirection::Row,
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            margin: UiRect::top(Val::Auto),
                                            padding: UiRect::all(Val::Px(10.0)),
                                            ..default()
                                        })
                                        .with_children(|parent| {
                                            let buttons = vec![
                                                PlayerInputsDisplayEntityMarker::B,
                                                PlayerInputsDisplayEntityMarker::A,
                                            ];

                                            for button in buttons {
                                                parent
                                                    .spawn((
                                                        Node {
                                                            width: Val::Auto,
                                                            height: Val::Auto,
                                                            border: UiRect::all(Val::Px(1.0)),
                                                            margin: UiRect::horizontal(Val::Px(
                                                                5.0,
                                                            )),
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
            Text::new("GAME PAUSE\nPRESS START TO CONTINUE"),
            TextFont::from_font_size(60.0),
            TextColor::from(WHITE),
            TextLayout::new_with_justify(JustifyText::Center),
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
            Query<(Entity, &GameStatisticsEntityMarker)>,
            Query<(Entity, &GameModeEntityMarker)>,
            Query<Entity, With<GameStopwatchEntityMarker>>,
            Query<(Entity, &PieceStatisticsCounterEntityMarker)>,
            Query<Entity, With<DASCounterEntityMarker>>,
        )>,
        Query<(&mut BackgroundColor, &DASCounterBarEntityMarker)>,
    )>,
    mut tw: TextUiWriter,
    game_config: Res<GameConfig>,
    player_data: Res<PlayerData>,
) {
    if let Ok(entity) = query.p0().p0().single_mut() {
        *tw.text(entity, 0) = format!("{:03}", player_data.board.lines());
    }
    if let Ok(entity) = query.p0().p1().single_mut() {
        *tw.text(entity, 0) = game_config.scoring.format(player_data.board.score());
    }
    if let Ok(entity) = query.p0().p2().single_mut() {
        *tw.text(entity, 0) = format!("{:02}", player_data.board.level());
    }
    for (entity, marker) in query.p0().p3() {
        match marker.0 {
            0 => *tw.text(entity, 0) = format!("{:4}", player_data.board.burned_lines()),
            1 => *tw.text(entity, 0) = format!("{:4}", player_data.board.single_clear()),
            2 => *tw.text(entity, 0) = format!("{:4}", player_data.board.double_clear()),
            3 => *tw.text(entity, 0) = format!("{:4}", player_data.board.triple_clear()),
            4 => *tw.text(entity, 0) = format!("{:4}", player_data.board.tetris_clear()),
            5 => {
                let rate = (player_data.board.tetris_rate() * 100.0).round() as usize;
                *tw.text(entity, 0) = format!("{:3}%", rate);
                match rate {
                    0..50 => *tw.color(entity, 0) = RED.into(),
                    50..80 => *tw.color(entity, 0) = YELLOW.into(),
                    _ => *tw.color(entity, 0) = GREEN.into(),
                }
            }
            6 => {
                let drought = player_data.board.drought();
                *tw.text(entity, 0) = format!("{:4}", drought);
                match drought {
                    0..7 => *tw.color(entity, 0) = WHITE.into(),
                    7..14 => *tw.color(entity, 0) = YELLOW.into(),
                    _ => *tw.color(entity, 0) = RED.into(),
                }
            }
            7 => *tw.text(entity, 0) = format!("{:4}", player_data.board.max_drought()),
            _ => unreachable!(),
        }
    }
    for (entity, marker) in query.p0().p4() {
        match marker.0 {
            0 => *tw.text(entity, 0) = format!("{:3}", game_config.start_level),
            1 => *tw.text(entity, 0) = format!("{:3}", game_config.linecap.to_str_abbr()),
            2 => *tw.text(entity, 0) = format!("{:3}", game_config.transition.to_str_abbr()),
            3 => *tw.text(entity, 0) = format!("{:3}", game_config.gravity.to_str_abbr()),
            4 => *tw.text(entity, 0) = format!("{:3}", game_config.tv_system.to_str_abbr()),
            5 => *tw.text(entity, 0) = format!("{:3}", game_config.invisible.to_str_abbr()),
            6 => *tw.text(entity, 0) = format!("{:3}", game_config.seeding.to_str_abbr()),
            7 => *tw.text(entity, 0) = format!("{}", player_data.board.seed()),
            _ => unreachable!(),
        }
    }
    if let Ok(entity) = query.p0().p5().single_mut() {
        *tw.text(entity, 0) = format!("TIME {}", format_hhmmss(player_data.stopwatch.elapsed()));
    }
    for (entity, piece) in query.p0().p6().iter_mut() {
        *tw.text(entity, 0) = format!("{:03}", player_data.board.get_piece_count(piece.0));
    }

    let das_color = if player_data.das_timer.is_active() {
        GREEN
    } else {
        RED
    };
    let das_ticks = game_config
        .tv_system
        .duration_to_ticks(player_data.das_timer.elapsed());
    if let Ok(entity) = query.p0().p7().single_mut() {
        *tw.text(entity, 0) = format!("{:02}", das_ticks);
        *tw.color(entity, 0) = das_color.into();
    }
    for (mut bg_color, marker) in query.p1() {
        if marker.0 < das_ticks {
            *bg_color = das_color.into();
        } else {
            *bg_color = BLACK.into();
        }
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
            .any(|square| square.0 == marker.0 as i32 && square.1 == marker.1 as i32)
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

fn update_next_piece(
    query: Query<(&mut ImageNode, &NextPieceEntityMarker)>,
    player_data: &PlayerData,
    square_image_assets: &SquareImageAssets,
) {
    for (mut img, marker) in query {
        let piece = player_data.board.next_pieces()[marker.idx];
        if piece
            .to_squares()
            .iter()
            .any(|square| square.0 == marker.x && square.1 == marker.y)
        {
            img.image = square_image_assets.get_image(SquareImageSize::Standard, piece);
        } else {
            img.image = square_image_assets.get_image(SquareImageSize::Standard, Piece::X);
        }
    }
}

fn update_piece_count(
    mut query: Query<(&mut ImageNode, &PieceStatisticsEntityMarker)>,
    square_image_assets: &SquareImageAssets,
) {
    for (mut img, marker) in query.iter_mut() {
        if marker
            .piece
            .to_squares()
            .iter()
            .any(|square| square.0 == marker.x && square.1 == marker.y)
        {
            img.image = square_image_assets.get_image(SquareImageSize::Small, marker.piece);
        }
    }
}

mod state_player_init {
    use super::*;

    pub(super) fn init_system(
        mut query: ParamSet<(
            Query<(&mut ImageNode, &BoardSquareEntityMarker)>,
            Query<(&mut ImageNode, &NextPieceEntityMarker)>,
            Query<(&mut ImageNode, &PieceStatisticsEntityMarker)>,
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
        update_next_piece(query.p1(), &player_data, &square_image_assets);
        update_piece_count(query.p2(), &square_image_assets);
        player_phase.set(PlayerPhase::Dropping);
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
            Query<(&mut ImageNode, &BoardSquareEntityMarker)>,
            Query<&mut Visibility, With<PauseScreenEntityMarker>>,
            Query<(&mut BackgroundColor, &PlayerInputsDisplayEntityMarker)>,
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
            app_state.set(AppState::Splash);
            return;
        }

        if player_inputs.start.just_pressed {
            if let Ok(mut vis) = query.p1().single_mut() {
                *vis = Visibility::Visible;
            }
            game_state.set(GameState::Pause);
            return;
        }

        query.p2().iter_mut().for_each(|(mut bg_color, marker)| {
            let pressed = match marker {
                PlayerInputsDisplayEntityMarker::Left => player_inputs.left.pressed,
                PlayerInputsDisplayEntityMarker::Right => player_inputs.right.pressed,
                PlayerInputsDisplayEntityMarker::Up => player_inputs.up.pressed,
                PlayerInputsDisplayEntityMarker::Down => player_inputs.down.pressed,
                PlayerInputsDisplayEntityMarker::A => player_inputs.a.pressed,
                PlayerInputsDisplayEntityMarker::B => player_inputs.b.pressed,
            };
            if pressed {
                *bg_color = RED.into();
            } else {
                *bg_color = WHITE.into();
            }
        });

        let (moved, lr_moved, rotated) = handle_input(&player_inputs, &time, &mut player_data);
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
        if lr_moved {
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
        query: Query<(&mut ImageNode, &BoardSquareEntityMarker)>,
        mut play_sound: EventWriter<PlaySoundEvent>,
        mut game_state: ResMut<NextState<GameState>>,
        mut player_phase: ResMut<NextState<PlayerPhase>>,
        game_config: Res<GameConfig>,
        mut player_data: ResMut<PlayerData>,
        square_image_assets: Res<SquareImageAssets>,
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

                play_sound.write(PlaySoundEvent::GameOver);
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
                    0 => {
                        play_sound.write(PlaySoundEvent::LockCurrPiece);
                    }
                    1 | 2 | 3 => {
                        play_sound.write(PlaySoundEvent::LineClear);
                    }
                    4 => {
                        play_sound.write(PlaySoundEvent::TetrisClear);
                    }
                    _ => unreachable!(),
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
                for (mut img, coord) in query.p0().iter_mut() {
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
                let old_level = player_data.board.level();
                player_data.board.clear_lines();
                let new_level = player_data.board.level();
                if new_level > old_level {
                    play_sound.write(PlaySoundEvent::LevelUp);
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
            Query<&mut BackgroundColor, With<BackgroundFlickeringEntityMarker>>,
            Query<(&mut ImageNode, &BoardSquareEntityMarker)>,
            Query<(&mut ImageNode, &NextPieceEntityMarker)>,
            Query<(&mut ImageNode, &PieceStatisticsEntityMarker)>,
        )>,
        game_config: Res<GameConfig>,
        mut player_data: ResMut<PlayerData>,
        mut player_phase: ResMut<NextState<PlayerPhase>>,
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
            update_next_piece(query.p2(), &player_data, &square_image_assets);
            update_piece_count(query.p3(), &square_image_assets);

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
        mut query: Query<&mut Visibility, With<PauseScreenEntityMarker>>,
        mut play_sound: EventWriter<PlaySoundEvent>,
        mut game_state: ResMut<NextState<GameState>>,
        mut app_state: ResMut<NextState<AppState>>,
    ) {
        let player_inputs = PlayerInputs::with_keyboard(&keys)
            | PlayerInputs::with_gamepads(gamepads, *controller_mapping);

        if player_inputs.soft_reset {
            play_sound.write(PlaySoundEvent::StartGame);
            app_state.set(AppState::Splash);
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
            app_state.set(AppState::Splash);
            return;
        }

        if player_inputs.start.just_pressed {
            app_state.set(AppState::LevelMenu);
        }
    }
}
