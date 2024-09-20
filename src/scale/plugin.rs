use bevy::prelude::*;
use num_traits::FromPrimitive;

use crate::{
    app_state::AppState, game::transform::GameTransform,
    game_option_menu::transform::GameOptionMenuTransform,
    level_menu::transform::LevelMenuTransform, splash::transform::SplashTransform,
};

pub fn setup(app: &mut App) {
    app.insert_resource(ScaleFactor::default())
        .add_systems(Startup, change_scale_system)
        .add_systems(
            Update,
            (change_scale_system, restore_app_state_system)
                .chain()
                .run_if(in_state(AppState::ChangeScale)),
        );
}

#[derive(Default, Clone, Copy, FromPrimitive, Resource)]
pub enum ScaleFactor {
    #[default]
    S720 = 0,
    S1080,
    S1440,
    S1800,
    S2160,
    S3240,
    S4320,
}

impl ScaleFactor {
    pub fn enum_prev(&mut self) -> Option<Self> {
        FromPrimitive::from_i8(*self as i8 - 1).map(|n| std::mem::replace(self, n))
    }

    pub fn enum_next(&mut self) -> Option<Self> {
        FromPrimitive::from_i8(*self as i8 + 1).map(|n| std::mem::replace(self, n))
    }

    pub fn mul(&self) -> f32 {
        let base = 1080.0;

        match self {
            ScaleFactor::S720 => 720.0 / base,
            ScaleFactor::S1080 => 1080.0 / base,
            ScaleFactor::S1440 => 1440.0 / base,
            ScaleFactor::S1800 => 1800.0 / base,
            ScaleFactor::S2160 => 2160.0 / base,
            ScaleFactor::S3240 => 3240.0 / base,
            ScaleFactor::S4320 => 4320.0 / base,
        }
    }
}

fn change_scale_system(
    scale_factor: Res<ScaleFactor>,
    mut splash_transform: ResMut<SplashTransform>,
    mut game_option_menu_transform: ResMut<GameOptionMenuTransform>,
    mut level_menu_transform: ResMut<LevelMenuTransform>,
    mut game_transform: ResMut<GameTransform>,
) {
    *splash_transform = SplashTransform::new(scale_factor.mul());
    *game_option_menu_transform = GameOptionMenuTransform::new(scale_factor.mul());
    *level_menu_transform = LevelMenuTransform::new(scale_factor.mul());
    *game_transform = GameTransform::new(scale_factor.mul());
}

fn restore_app_state_system(mut app_state: ResMut<NextState<AppState>>) {
    app_state.set(AppState::GameModeMenu);
}
