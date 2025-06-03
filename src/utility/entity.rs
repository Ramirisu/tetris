use bevy::prelude::*;

pub fn despawn_all<T: Component>(q: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &q {
        commands.entity(entity).despawn();
    }
}
