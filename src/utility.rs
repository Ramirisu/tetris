use bevy::prelude::*;

pub fn clear_screen<T: Component>(query: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
