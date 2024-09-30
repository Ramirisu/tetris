use bevy::prelude::*;

use super::controller_mapping::ControllerMapping;

pub fn setup(app: &mut App) {
    app.insert_resource(ControllerMapping::default());
}
