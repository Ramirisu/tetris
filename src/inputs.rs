use bevy::prelude::*;

use crate::controller::Controller;

pub fn setup(app: &mut App) {
    app.insert_resource(ControllerType::default());
}

#[derive(Default, Clone, Copy, Resource)]
pub enum ControllerType {
    #[default]
    TypeA,
    TypeB,
}

#[derive(Clone, Copy)]
pub struct PlayerInputs {
    pub up: (bool, bool),
    pub down: (bool, bool),
    pub left: (bool, bool),
    pub right: (bool, bool),
    pub a: (bool, bool),
    pub b: (bool, bool),
    pub start: bool,
    pub select: bool,

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
            start: false,
            select: false,
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
            start: inputs.just_pressed(KeyCode::Enter),
            select: inputs.just_pressed(KeyCode::ShiftLeft),
            soft_reset: inputs.just_pressed(KeyCode::Escape),
        }
    }

    pub fn with_gamepads(
        buttons: &ButtonInput<GamepadButton>,
        controller: &Controller,
        controller_type: ControllerType,
    ) -> Self {
        let mut inputs = Self::new();
        for gamepad in &controller.gamepads {
            inputs |= Self::with_gamepad(buttons, *gamepad, controller_type);
        }
        inputs
    }

    fn with_gamepad(
        buttons: &ButtonInput<GamepadButton>,
        gamepad: Gamepad,
        controller_type: ControllerType,
    ) -> Self {
        match controller_type {
            ControllerType::TypeA => Self::with_gamepad_type_a(buttons, gamepad),
            ControllerType::TypeB => Self::with_gamepad_type_b(buttons, gamepad),
        }
    }

    fn with_gamepad_type_a(buttons: &ButtonInput<GamepadButton>, gamepad: Gamepad) -> Self {
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
            start: buttons.just_pressed(Self::gamepad_button(gamepad, GamepadButtonType::Start)),
            select: buttons.just_pressed(Self::gamepad_button(gamepad, GamepadButtonType::Select)),
            soft_reset: buttons.pressed(Self::gamepad_button(gamepad, GamepadButtonType::Select))
                && buttons.pressed(Self::gamepad_button(gamepad, GamepadButtonType::Start))
                && buttons.pressed(Self::gamepad_button(gamepad, GamepadButtonType::East))
                && buttons.pressed(Self::gamepad_button(gamepad, GamepadButtonType::South)),
        }
    }

    fn with_gamepad_type_b(buttons: &ButtonInput<GamepadButton>, gamepad: Gamepad) -> Self {
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
                buttons.just_pressed(Self::gamepad_button(gamepad, GamepadButtonType::South)),
                buttons.pressed(Self::gamepad_button(gamepad, GamepadButtonType::South)),
            ),
            b: (
                buttons.just_pressed(Self::gamepad_button(gamepad, GamepadButtonType::West)),
                buttons.pressed(Self::gamepad_button(gamepad, GamepadButtonType::West)),
            ),
            start: buttons.just_pressed(Self::gamepad_button(gamepad, GamepadButtonType::Start)),
            select: buttons.just_pressed(Self::gamepad_button(gamepad, GamepadButtonType::Select)),
            soft_reset: buttons.pressed(Self::gamepad_button(gamepad, GamepadButtonType::Select))
                && buttons.pressed(Self::gamepad_button(gamepad, GamepadButtonType::Start))
                && buttons.pressed(Self::gamepad_button(gamepad, GamepadButtonType::South))
                && buttons.pressed(Self::gamepad_button(gamepad, GamepadButtonType::West)),
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
            start: self.start | rhs.start,
            select: self.select | rhs.select,
            soft_reset: self.soft_reset | rhs.soft_reset,
        }
    }
}
