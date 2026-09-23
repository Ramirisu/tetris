use bevy::{ecs::system::NonSendMarker, prelude::*, window::PrimaryWindow, winit::WINIT_WINDOWS};
use image::{DynamicImage, GenericImageView, ImageBuffer};
use strum::IntoEnumIterator;
use winit::window::Icon;

use crate::{
    app_state::AppState,
    game::{
        level::Level,
        palette::{SquareImageSize, get_square_image_by_level},
        piece::Piece,
    },
    screens::language::{Language, LanguageScreenData},
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            init_app_font_system,
            init_app_icon_system,
            init_app_locale_system,
            complete_init_system,
        )
            .chain()
            .run_if(in_state(AppState::Init)),
    );
}

fn init_app_font_system(mut fonts: ResMut<Assets<Font>>) {
    const MONO_FONT_BYTES: &[u8] = include_bytes!("../assets/fonts/NotoSansCJK-Regular.ttc");
    let font = Font::from_bytes(MONO_FONT_BYTES.to_vec());

    fonts
        .insert(AssetId::default(), font)
        .expect("Impossible to change default font");
}

fn init_app_icon_system(
    mut primary_window: Query<Entity, With<PrimaryWindow>>,
    _marker: NonSendMarker,
) {
    let Ok(window) = primary_window.single_mut() else {
        warn!("Unable to get entity of primary window");
        return;
    };

    WINIT_WINDOWS.with_borrow(|winit_windows| {
        let Some(primary) = winit_windows.get_window(window) else {
            warn!("Unable to get primary window");
            return;
        };

        const SIZE: u32 = 256;
        let image = create_app_icon().resize(SIZE, SIZE, image::imageops::FilterType::Nearest);
        let Ok(icon) = Icon::from_rgba(image.into_rgba8().into_vec(), SIZE, SIZE) else {
            error!("Failed to convert square image into `Icon`");
            return;
        };

        primary.set_window_icon(Some(icon));
    });
}

fn create_app_icon() -> DynamicImage {
    let sqr = get_square_image_by_level(SquareImageSize::Small, Piece::j(), Level(19));
    let (width, height) = sqr.dimensions();
    const PATTERN: [[u8; 3]; 4] = [
        [0, 0, 0], //
        [1, 1, 1],
        [0, 1, 0],
        [0, 0, 0],
    ];

    let mut buffer = ImageBuffer::new(
        PATTERN[0].len() as u32 * width,
        PATTERN.len() as u32 * height,
    );
    draw_icon_pattern(&mut buffer, &sqr, &PATTERN, width, height);

    let image: DynamicImage = buffer.into();
    image.crop_imm(0, height / 2, width * 3, height * 3)
}

fn draw_icon_pattern(
    buffer: &mut ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    square: &image::DynamicImage,
    pattern: &[[u8; 3]; 4],
    width: u32,
    height: u32,
) {
    for (pattern_y, row) in pattern.iter().enumerate() {
        for (pattern_x, &filled) in row.iter().enumerate() {
            if filled == 0 {
                continue;
            }

            for y in 0..height {
                for x in 0..width {
                    buffer.put_pixel(
                        x + pattern_x as u32 * width,
                        y + pattern_y as u32 * height,
                        square.get_pixel(x, y),
                    );
                }
            }
        }
    }
}

fn init_app_locale_system(mut language_screen_data: ResMut<LanguageScreenData>) {
    if let Some(locale) = sys_locale::get_locale() {
        info!("System locale: {}", locale);
        rust_i18n::set_locale(&locale);
        for lang in Language::iter() {
            if lang.locale() == locale {
                language_screen_data.selected_lang = lang;
            }
        }
    } else {
        warn!("Unable to get system locale");
    }
}

fn complete_init_system(mut app_state: ResMut<NextState<AppState>>) {
    app_state.set(AppState::LoadingScreen);
}
