use bevy::{
    input::gamepad::{GamepadConnection, GamepadEvent},
    prelude::*,
};

pub fn setup(app: &mut App) {
    app.insert_resource(Controller::default())
        .add_systems(Update, controller_connection_system);
}

#[derive(Resource, Default)]
pub struct Controller {
    pub gamepads: Vec<Gamepad>,
}

impl Controller {
    pub fn insert(&mut self, target: Gamepad) {
        match self
            .gamepads
            .binary_search_by(|gamepad| gamepad.id.cmp(&target.id))
        {
            Ok(_) => (),
            Err(pos) => self.gamepads.insert(pos, target),
        };
    }

    pub fn remove(&mut self, target: Gamepad) {
        match self
            .gamepads
            .binary_search_by(|gamepad| gamepad.id.cmp(&target.id))
        {
            Ok(pos) => {
                self.gamepads.remove(pos);
            }
            Err(_) => (),
        };
    }
}

fn controller_connection_system(
    mut controller: ResMut<Controller>,
    mut event_reader: EventReader<GamepadEvent>,
) {
    for ev in event_reader.read() {
        let GamepadEvent::Connection(event) = ev else {
            continue;
        };
        match &event.connection {
            GamepadConnection::Connected(_) => controller.insert(event.gamepad),
            GamepadConnection::Disconnected => controller.remove(event.gamepad),
        }
    }
}
