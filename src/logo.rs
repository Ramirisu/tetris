use bevy::{ecs::spawn, prelude::*};

use crate::game::{
    level::Level,
    palette::{SquareImageSize, get_square_image_by_level, into_image},
    piece::Piece,
};

pub fn logo(size: Val, image_assets: &mut Assets<Image>) -> impl Bundle {
    let logo_images = load_logo_images(image_assets);

    (
        Node {
            display: Display::Grid,
            grid_template_columns: vec![GridTrack::auto(); TETRIS_BITMAP[0].len()],
            ..default()
        },
        Children::spawn(spawn::SpawnIter(TETRIS_BITMAP.iter().flat_map(
            move |rows| {
                let logo_images = logo_images.clone();
                rows.iter().map(move |sqr| {
                    (
                        Node {
                            width: size,
                            height: size,
                            ..default()
                        },
                        ImageNode::new(logo_images[(*sqr) as usize].clone()),
                    )
                })
            },
        ))),
    )
}

fn load_logo_images(image_assets: &mut Assets<Image>) -> [Handle<Image>; 7] {
    [
        image_assets.add(into_image(get_square_image_by_level(
            SquareImageSize::Small,
            Piece::X,
            Level(0),
        ))),
        image_assets.add(into_image(get_square_image_by_level(
            SquareImageSize::Small,
            Piece::j(),
            Level(8),
        ))),
        image_assets.add(into_image(get_square_image_by_level(
            SquareImageSize::Small,
            Piece::t(),
            Level(2),
        ))),
        image_assets.add(into_image(get_square_image_by_level(
            SquareImageSize::Small,
            Piece::z(),
            Level(8),
        ))),
        image_assets.add(into_image(get_square_image_by_level(
            SquareImageSize::Small,
            Piece::z(),
            Level(9),
        ))),
        image_assets.add(into_image(get_square_image_by_level(
            SquareImageSize::Small,
            Piece::z(),
            Level(0),
        ))),
        image_assets.add(into_image(get_square_image_by_level(
            SquareImageSize::Small,
            Piece::z(),
            Level(1),
        ))),
    ]
}

#[rustfmt::skip]
const TETRIS_BITMAP: &[[u8; 21]; 5] = &[
    [1, 1, 1, 0, 2, 2, 2, 0, 3, 3, 3, 0, 4, 4, 4, 0, 5, 0, 6, 6, 6],
    [0, 1, 0, 0, 2, 0, 0, 0, 0, 3, 0, 0, 4, 0, 4, 0, 5, 0, 6, 0, 0],
    [0, 1, 0, 0, 2, 2, 2, 0, 0, 3, 0, 0, 4, 4, 4, 0, 5, 0, 6, 6, 6],
    [0, 1, 0, 0, 2, 0, 0, 0, 0, 3, 0, 0, 4, 4, 0, 0, 5, 0, 0, 0, 6],
    [0, 1, 0, 0, 2, 2, 2, 0, 0, 3, 0, 0, 4, 0, 4, 0, 5, 0, 6, 6, 6],
];
