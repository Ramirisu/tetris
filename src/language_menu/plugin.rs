use bevy::{
    color::palettes::css::{BLUE, WHITE},
    prelude::*,
};
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount, EnumIter, FromRepr};

use crate::{
    app_state::AppState,
    audio::plugin::PlaySoundEvent,
    enum_advance, enum_advance_cycle,
    input::{controller_mapping::ControllerMapping, player_inputs::PlayerInputs},
    logo::{TETRIS_BITMAP, load_logo_images},
    utility::despawn_all,
};

pub fn setup(app: &mut App) {
    app.insert_resource(LanguageMunuData::default())
        .add_systems(OnEnter(AppState::LanguageMenu), setup_screen)
        .add_systems(
            Update,
            (handle_input_system, update_ui_system).run_if(in_state(AppState::LanguageMenu)),
        )
        .add_systems(
            OnExit(AppState::LanguageMenu),
            despawn_all::<LanguageMenuEntityMarker>,
        );
}

#[derive(Component)]
struct LanguageMenuEntityMarker;

#[derive(Component)]
struct LanguageSelectionEntityMarker(Language);

#[derive(Default, Clone, Copy, PartialEq, Eq, FromRepr, EnumIter, EnumCount)]
enum Language {
    #[default]
    English,
    TraditionalChinese,
    SimplifiedChinese,
}

enum_advance::enum_advance_derive!(Language);
enum_advance_cycle::enum_advance_cycle_derive!(Language);

impl Language {
    pub fn name(&self) -> &'static str {
        match *self {
            Language::English => "English",
            Language::TraditionalChinese => "繁體中文",
            Language::SimplifiedChinese => "简体中文",
        }
    }

    pub fn locale(&self) -> &'static str {
        match *self {
            Language::English => "en",
            Language::TraditionalChinese => "zh-TW",
            Language::SimplifiedChinese => "zh-CN",
        }
    }
}

#[derive(Default, Resource)]
struct LanguageMunuData {
    language_selection: Language,
}

fn setup_screen(mut commands: Commands, mut image_assets: ResMut<Assets<Image>>) {
    let logo_images = load_logo_images(&mut image_assets);

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
            LanguageMenuEntityMarker,
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    display: Display::Grid,
                    grid_template_columns: vec![GridTrack::auto(); TETRIS_BITMAP[0].len()],
                    margin: UiRect::all(Val::Px(30.0)),
                    ..default()
                })
                .with_children(|parent| {
                    TETRIS_BITMAP.iter().for_each(|rows| {
                        rows.iter().for_each(|sqr| {
                            parent.spawn((
                                Node {
                                    width: Val::Px(30.0),
                                    height: Val::Px(30.0),
                                    ..default()
                                },
                                ImageNode::new(logo_images[(*sqr) as usize].clone()),
                            ));
                        })
                    });
                });

            parent
                .spawn((
                    Node {
                        width: Val::Px(400.0),
                        height: Val::Auto,
                        display: Display::Grid,
                        grid_template_columns: vec![GridTrack::auto(); 2],
                        column_gap: Val::Px(20.0),
                        row_gap: Val::Px(5.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(20.0)),
                        padding: UiRect::all(Val::Px(20.0)),
                        border: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    BorderColor::from(BLUE),
                ))
                .with_children(|parent| {
                    for lang in Language::iter() {
                        parent.spawn((
                            Text::default(),
                            TextFont::from_font_size(40.0),
                            TextColor::from(WHITE),
                            TextLayout::new_with_justify(JustifyText::Center),
                            LanguageSelectionEntityMarker(lang),
                        ));
                        parent.spawn((
                            Text::new(lang.name()),
                            TextFont::from_font_size(40.0),
                            TextColor::from(WHITE),
                            TextLayout::new_with_justify(JustifyText::Left),
                        ));
                    }
                });
        });
}

fn handle_input_system(
    keys: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    controller_mapping: Res<ControllerMapping>,
    mut lang_menu_data: ResMut<LanguageMunuData>,
    mut play_sound: EventWriter<PlaySoundEvent>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    let player_inputs = PlayerInputs::with_keyboard(&keys)
        | PlayerInputs::with_gamepads(gamepads, *controller_mapping);

    if player_inputs.start.just_pressed {
        rust_i18n::set_locale(lang_menu_data.language_selection.locale());
        play_sound.write(PlaySoundEvent::StartGame);
        app_state.set(AppState::Splash);
        return;
    }

    match (
        player_inputs.up.just_pressed,
        player_inputs.down.just_pressed,
    ) {
        (false, true) => {
            lang_menu_data.language_selection = lang_menu_data.language_selection.enum_next_cycle();
            play_sound.write(PlaySoundEvent::MoveCursor);
        }
        (true, false) => {
            lang_menu_data.language_selection = lang_menu_data.language_selection.enum_prev_cycle();
            play_sound.write(PlaySoundEvent::MoveCursor);
        }
        _ => (),
    }
}

fn update_ui_system(
    query: Query<(Entity, &LanguageSelectionEntityMarker)>,
    mut tw: TextUiWriter,
    lang_menu_data: Res<LanguageMunuData>,
) {
    for (entity, marker) in query {
        *tw.text(entity, 0) = (if lang_menu_data.language_selection == marker.0 {
            ">>"
        } else {
            ""
        })
        .into();
    }
}
