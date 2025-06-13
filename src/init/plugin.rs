use bevy::{prelude::*, window::PrimaryWindow, winit::WinitWindows};
use image::{DynamicImage, GenericImageView, ImageBuffer};
use strum::IntoEnumIterator;
use winit::window::Icon;

use crate::{
    app_state::AppState,
    game_screen::{
        level::Level,
        palette::{SquareImageSize, get_square_image_by_level},
        piece::Piece,
    },
    language_menu::plugin::{Language, LanguageMenuData},
};

pub fn setup(app: &mut App) {
    app.add_systems(
        Update,
        (
            init_app_icon_system,
            init_app_locale_system,
            complete_initialization_system,
        )
            .chain()
            .run_if(in_state(AppState::Init)),
    );
}

fn init_app_icon_system(
    mut primary_window: Query<Entity, With<PrimaryWindow>>,
    windows: NonSend<WinitWindows>,
) {
    let Ok(window) = primary_window.single_mut() else {
        return;
    };

    let Some(primary) = windows.get_window(window) else {
        return;
    };

    const SIZE: u32 = 256;
    let image = create_app_icon().resize(SIZE, SIZE, image::imageops::FilterType::Nearest);
    let Ok(icon) = Icon::from_rgba(image.into_rgba8().into_vec(), SIZE, SIZE) else {
        error!("Failed to convert square image into `Icon`");
        return;
    };

    primary.set_window_icon(Some(icon));
}

fn create_app_icon() -> DynamicImage {
    let sqr = get_square_image_by_level(SquareImageSize::Small, Piece::j(), Level(19));
    let (width, height) = sqr.dimensions();
    let pattern = [
        [0, 0, 0], //
        [1, 1, 1],
        [0, 1, 0],
        [0, 0, 0],
    ];

    let mut buffer = ImageBuffer::new(
        pattern[0].len() as u32 * width,
        pattern.len() as u32 * height,
    );

    for (py, row) in pattern.iter().enumerate() {
        for (px, element) in row.iter().enumerate() {
            for y in 0..height {
                for x in 0..width {
                    if *element > 0 {
                        buffer.put_pixel(
                            x + px as u32 * width,
                            y + py as u32 * height,
                            sqr.get_pixel(x, y),
                        );
                    }
                }
            }
        }
    }

    let image: DynamicImage = buffer.into();
    image.crop_imm(0, height / 2, width * 3, height * 3)
}

fn init_app_locale_system(mut lang_menu_data: ResMut<LanguageMenuData>) {
    if let Some(locale) = sys_locale::get_locale() {
        info!("System locale: {}", locale);
        rust_i18n::set_locale(&locale);
        for lang in Language::iter() {
            if lang.locale() == locale {
                lang_menu_data.selected_lang = lang;
            }
        }
    } else {
        warn!("Unable to get system locale");
    }
}

fn complete_initialization_system(mut app_state: ResMut<NextState<AppState>>) {
    app_state.set(AppState::LoadingScreen);
}
