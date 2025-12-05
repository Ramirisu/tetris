use bevy::{
    color::palettes::css::{BLUE, WHITE},
    ecs::spawn::SpawnWith,
    prelude::*,
};
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount, EnumIter, FromRepr};

use crate::{
    app_state::AppState,
    audio::plugin::PlaySoundMessage,
    input::{controller_mapping::ControllerMapping, player_inputs::PlayerInputs},
    logo::logo,
    settings_menu::scale_factor::{WINDOW_HEIGHT, WINDOW_WIDTH},
    utility::{effect::flicker, entity::despawn_all, enum_advance, enum_advance_cycle},
};

pub fn setup(app: &mut App) {
    app.insert_resource(LanguageMenuData::default())
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
pub enum Language {
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
pub struct LanguageMenuData {
    pub selected_lang: Language,
}

fn setup_screen(mut commands: Commands, mut image_assets: ResMut<Assets<Image>>) {
    commands.spawn((
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
        Children::spawn(Spawn((
            Node {
                width: Val::Px(WINDOW_WIDTH),
                height: Val::Px(WINDOW_HEIGHT),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(50.0)),
                ..default()
            },
            Children::spawn((
                Spawn((
                    Node {
                        margin: UiRect::all(Val::Px(40.0)),
                        ..default()
                    },
                    Children::spawn(Spawn(logo(Val::Px(20.0), &mut image_assets))),
                )),
                Spawn((
                    Node {
                        width: Val::Px(300.0),
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
                    Children::spawn(SpawnWith(|p: &mut ChildSpawner| {
                        for lang in Language::iter() {
                            p.spawn((
                                Text::new("▶"),
                                TextFont::from_font_size(25.0),
                                TextColor::from(WHITE),
                                TextLayout::new_with_justify(Justify::Center),
                                LanguageSelectionEntityMarker(lang),
                            ));
                            p.spawn((
                                Text::new(lang.name()),
                                TextFont::from_font_size(35.0),
                                TextColor::from(WHITE),
                                TextLayout::new_with_justify(Justify::Left),
                            ));
                        }
                    })),
                )),
            )),
        ))),
    ));
}

fn handle_input_system(
    keys: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    controller_mapping: Res<ControllerMapping>,
    mut lang_menu_data: ResMut<LanguageMenuData>,
    mut play_sound: MessageWriter<PlaySoundMessage>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    let player_inputs = PlayerInputs::with_keyboard(&keys)
        | PlayerInputs::with_gamepads(gamepads, *controller_mapping);

    if player_inputs.start.just_pressed {
        rust_i18n::set_locale(lang_menu_data.selected_lang.locale());
        play_sound.write(PlaySoundMessage::StartGame);
        app_state.set(AppState::SettingsMenu);
        return;
    }

    if player_inputs.soft_reset || player_inputs.b.just_pressed {
        play_sound.write(PlaySoundMessage::StartGame);
        app_state.set(AppState::SplashScreen);
        return;
    }

    match (
        player_inputs.up.just_pressed,
        player_inputs.down.just_pressed,
    ) {
        (false, true) => {
            lang_menu_data.selected_lang = lang_menu_data.selected_lang.enum_next_cycle();
            play_sound.write(PlaySoundMessage::MoveCursor);
        }
        (true, false) => {
            lang_menu_data.selected_lang = lang_menu_data.selected_lang.enum_prev_cycle();
            play_sound.write(PlaySoundMessage::MoveCursor);
        }
        _ => (),
    }
}

fn update_ui_system(
    t: Res<Time>,
    q: Query<(Entity, &LanguageSelectionEntityMarker)>,
    mut tw: TextUiWriter,
    lang_menu_data: Res<LanguageMenuData>,
) {
    for (entity, marker) in q {
        tw.color(entity, 0)
            .set_alpha(if lang_menu_data.selected_lang == marker.0 {
                flicker(t.elapsed_secs(), 0.5)
            } else {
                0.0
            });
    }
}
