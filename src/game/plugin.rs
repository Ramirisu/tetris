use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{game_state::GameState, utility::clear_screen};

pub fn setup(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), setup_screen)
        .add_systems(OnExit(GameState::Game), clear_screen::<OnGameScreen>);
}

const WIDTH: f32 = 400.0;
const HEIGHT: f32 = 800.0;

#[derive(Component)]
struct OnGameScreen;

fn setup_screen(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(WIDTH, HEIGHT))),
            material: materials.add(Color::srgb(0.0, 0.0, 0.0)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        OnGameScreen,
    ));
}
