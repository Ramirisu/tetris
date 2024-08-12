use bevy::{
    color::palettes::css::{BLACK, CRIMSON},
    prelude::*,
};

use crate::{app_state::AppState, utility::despawn_all};

pub fn setup(app: &mut App) {
    app.add_systems(OnEnter(AppState::Menu), setup_screen)
        .add_systems(Update, menu_action)
        .add_systems(OnExit(AppState::Menu), despawn_all::<OnMenuScreen>);
}

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

#[derive(Component)]
struct OnMenuScreen;

fn setup_screen(mut commands: Commands) {
    let button_style = Style {
        width: Val::Px(200.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font_size: 40.0,
        color: TEXT_COLOR,
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: CRIMSON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            "TETRIS",
                            TextStyle {
                                font_size: 80.0,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                    );

                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: BLACK.into(),
                                ..default()
                            },
                            MenuButtonAction::Start,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Start",
                                button_text_style.clone(),
                            ));
                        });
                });
        });
}

#[derive(Clone, Copy, Component)]
enum MenuButtonAction {
    Start,
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Start => app_state.set(AppState::Game),
            }
        }
    }
}
