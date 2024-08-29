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

fn controller_connection_system(
    mut controller: ResMut<Controller>,
    mut event_reader: EventReader<GamepadEvent>,
) {
    for ev in event_reader.read() {
        let GamepadEvent::Connection(event) = ev else {
            continue;
        };
        match &event.connection {
            GamepadConnection::Connected(_) => {
                match controller
                    .gamepads
                    .binary_search_by(|gamepad| gamepad.id.cmp(&event.gamepad.id))
                {
                    Ok(_) => (),
                    Err(pos) => controller.gamepads.insert(pos, event.gamepad),
                };
            }
            GamepadConnection::Disconnected => {
                match controller
                    .gamepads
                    .binary_search_by(|gamepad| gamepad.id.cmp(&event.gamepad.id))
                {
                    Ok(pos) => {
                        controller.gamepads.remove(pos);
                    }
                    Err(_) => (),
                };
            }
        }
    }
}
