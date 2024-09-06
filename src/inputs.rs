use bevy::prelude::*;

use crate::controller::Controller;

pub fn setup(app: &mut App) {
    app.insert_resource(PlayerInputs::default())
        .add_systems(Update, update_player_inputs);
}

fn update_player_inputs(
    keys: Res<ButtonInput<KeyCode>>,
    buttons: Res<ButtonInput<GamepadButton>>,
    controller: Res<Controller>,
    mut player_inputs: ResMut<PlayerInputs>,
) {
    *player_inputs =
        PlayerInputs::with_keyboard(&keys) | PlayerInputs::with_gamepads(&buttons, &controller);
}

#[derive(Clone, Copy, Resource)]
pub struct PlayerInputs {
    pub up: (bool, bool),
    pub down: (bool, bool),
    pub left: (bool, bool),
    pub right: (bool, bool),
    pub a: (bool, bool), // DPad::East
    pub b: (bool, bool), // DPad::South
    pub x: (bool, bool), // DPad::North
    pub y: (bool, bool), // DPad::West
    pub start: bool,
    pub select: bool,

    pub toggle_fullscreen: bool,
    pub soft_reset: bool,
}

impl PlayerInputs {
    pub fn new() -> Self {
        Self {
            up: (false, false),
            down: (false, false),
            left: (false, false),
            right: (false, false),
            a: (false, false),
            b: (false, false),
            x: (false, false),
            y: (false, false),
            start: false,
            select: false,
            toggle_fullscreen: false,
            soft_reset: false,
        }
    }

    pub fn with_keyboard(inputs: &ButtonInput<KeyCode>) -> Self {
        Self {
            up: (
                inputs.just_pressed(KeyCode::ArrowUp),
                inputs.pressed(KeyCode::ArrowUp),
            ),
            down: (
                inputs.just_pressed(KeyCode::ArrowDown),
                inputs.pressed(KeyCode::ArrowDown),
            ),
            left: (
                inputs.just_pressed(KeyCode::ArrowLeft),
                inputs.pressed(KeyCode::ArrowLeft),
            ),
            right: (
                inputs.just_pressed(KeyCode::ArrowRight),
                inputs.pressed(KeyCode::ArrowRight),
            ),
            a: (
                inputs.just_pressed(KeyCode::KeyX),
                inputs.pressed(KeyCode::KeyX),
            ),
            b: (
                inputs.just_pressed(KeyCode::KeyZ),
                inputs.pressed(KeyCode::KeyZ),
            ),
            x: (
                inputs.just_pressed(KeyCode::KeyS),
                inputs.pressed(KeyCode::KeyS),
            ),
            y: (
                inputs.just_pressed(KeyCode::KeyA),
                inputs.pressed(KeyCode::KeyA),
            ),
            start: inputs.just_pressed(KeyCode::Enter),
            select: inputs.just_pressed(KeyCode::ShiftLeft),
            toggle_fullscreen: inputs.just_pressed(KeyCode::F11),
            soft_reset: inputs.just_pressed(KeyCode::ShiftLeft)
                || inputs.just_pressed(KeyCode::Escape),
        }
    }

    pub fn with_gamepads(buttons: &ButtonInput<GamepadButton>, controller: &Controller) -> Self {
        let mut inputs = Self::new();
        for gamepad in &controller.gamepads {
            inputs |= Self::with_gamepad(buttons, *gamepad);
        }
        inputs
    }

    fn with_gamepad(buttons: &ButtonInput<GamepadButton>, gamepad: Gamepad) -> Self {
        Self {
            up: (
                buttons.just_pressed(Self::gamepad_button(gamepad, GamepadButtonType::DPadUp)),
                buttons.pressed(Self::gamepad_button(gamepad, GamepadButtonType::DPadUp)),
            ),
            down: (
                buttons.just_pressed(Self::gamepad_button(gamepad, GamepadButtonType::DPadDown)),
                buttons.pressed(Self::gamepad_button(gamepad, GamepadButtonType::DPadDown)),
            ),
            left: (
                buttons.just_pressed(Self::gamepad_button(gamepad, GamepadButtonType::DPadLeft)),
                buttons.pressed(Self::gamepad_button(gamepad, GamepadButtonType::DPadLeft)),
            ),
            right: (
                buttons.just_pressed(Self::gamepad_button(gamepad, GamepadButtonType::DPadRight)),
                buttons.pressed(Self::gamepad_button(gamepad, GamepadButtonType::DPadRight)),
            ),
            a: (
                buttons.just_pressed(Self::gamepad_button(gamepad, GamepadButtonType::East)),
                buttons.pressed(Self::gamepad_button(gamepad, GamepadButtonType::East)),
            ),
            b: (
                buttons.just_pressed(Self::gamepad_button(gamepad, GamepadButtonType::South)),
                buttons.pressed(Self::gamepad_button(gamepad, GamepadButtonType::South)),
            ),
            x: (
                buttons.just_pressed(Self::gamepad_button(gamepad, GamepadButtonType::North)),
                buttons.pressed(Self::gamepad_button(gamepad, GamepadButtonType::North)),
            ),
            y: (
                buttons.just_pressed(Self::gamepad_button(gamepad, GamepadButtonType::West)),
                buttons.pressed(Self::gamepad_button(gamepad, GamepadButtonType::West)),
            ),
            start: buttons.just_pressed(Self::gamepad_button(gamepad, GamepadButtonType::Start)),
            select: buttons.just_pressed(Self::gamepad_button(gamepad, GamepadButtonType::Select)),
            toggle_fullscreen: false,
            soft_reset: buttons
                .just_pressed(Self::gamepad_button(gamepad, GamepadButtonType::Select)),
        }
    }

    fn gamepad_button(gamepad: Gamepad, button_type: GamepadButtonType) -> GamepadButton {
        GamepadButton {
            gamepad,
            button_type,
        }
    }
}

impl Default for PlayerInputs {
    fn default() -> Self {
        Self::new()
    }
}

impl std::ops::BitOrAssign for PlayerInputs {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl std::ops::BitOr for PlayerInputs {
    type Output = PlayerInputs;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            up: (self.up.0 | rhs.up.0, self.up.1 | rhs.up.1),
            down: (self.down.0 | rhs.down.0, self.down.1 | rhs.down.1),
            left: (self.left.0 | rhs.left.0, self.left.1 | rhs.left.1),
            right: (self.right.0 | rhs.right.0, self.right.1 | rhs.right.1),
            a: (self.a.0 | rhs.a.0, self.a.1 | rhs.a.1),
            b: (self.b.0 | rhs.b.0, self.b.1 | rhs.b.1),
            x: (self.x.0 | rhs.x.0, self.x.1 | rhs.x.1),
            y: (self.y.0 | rhs.y.0, self.y.1 | rhs.y.1),
            start: self.start | rhs.start,
            select: self.select | rhs.select,
            toggle_fullscreen: self.toggle_fullscreen | rhs.toggle_fullscreen,
            soft_reset: self.soft_reset | rhs.soft_reset,
        }
    }
}
