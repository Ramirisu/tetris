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
    pub gamepad: Option<Gamepad>,
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
                if controller.gamepad.is_none() {
                    controller.gamepad = Some(event.gamepad)
                }
            }
            GamepadConnection::Disconnected => {
                if let Some(gamepad) = controller.gamepad {
                    if gamepad == event.gamepad {
                        controller.gamepad = None;
                    }
                }
            }
        }
    }
}
