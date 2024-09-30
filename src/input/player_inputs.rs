use bevy::prelude::*;

use crate::controller::Controller;

use super::controller_mapping::ControllerMapping;

#[derive(Clone, Copy)]
pub struct PlayerInput {
    pub just_pressed: bool,
    pub pressed: bool,
}

impl PlayerInput {
    pub fn new() -> Self {
        Self {
            just_pressed: false,
            pressed: false,
        }
    }
}

impl Default for PlayerInput {
    fn default() -> Self {
        Self::new()
    }
}

impl std::ops::BitOrAssign for PlayerInput {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl std::ops::BitOr for PlayerInput {
    type Output = PlayerInput;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            just_pressed: self.just_pressed | rhs.just_pressed,
            pressed: self.pressed | rhs.pressed,
        }
    }
}

#[derive(Clone, Copy)]
pub struct PlayerInputs {
    pub up: PlayerInput,
    pub down: PlayerInput,
    pub left: PlayerInput,
    pub right: PlayerInput,
    pub a: PlayerInput,
    pub b: PlayerInput,
    pub start: PlayerInput,
    pub select: PlayerInput,
    pub soft_reset: bool,
}

impl PlayerInputs {
    pub fn new() -> Self {
        Self {
            up: PlayerInput::default(),
            down: PlayerInput::default(),
            left: PlayerInput::default(),
            right: PlayerInput::default(),
            a: PlayerInput::default(),
            b: PlayerInput::default(),
            start: PlayerInput::default(),
            select: PlayerInput::default(),
            soft_reset: false,
        }
    }

    pub fn with_keyboard(inputs: &ButtonInput<KeyCode>) -> Self {
        Self {
            up: Self::from_keyboard_key(KeyCode::ArrowUp, inputs),
            down: Self::from_keyboard_key(KeyCode::ArrowDown, inputs),
            left: Self::from_keyboard_key(KeyCode::ArrowLeft, inputs),
            right: Self::from_keyboard_key(KeyCode::ArrowRight, inputs),
            a: Self::from_keyboard_key(KeyCode::KeyX, inputs),
            b: Self::from_keyboard_key(KeyCode::KeyZ, inputs),
            start: Self::from_keyboard_key(KeyCode::Enter, inputs),
            select: Self::from_keyboard_key(KeyCode::ShiftLeft, inputs),
            soft_reset: inputs.just_pressed(KeyCode::Escape),
        }
    }

    fn from_keyboard_key(key: KeyCode, inputs: &ButtonInput<KeyCode>) -> PlayerInput {
        PlayerInput {
            just_pressed: inputs.just_pressed(key),
            pressed: inputs.pressed(key),
        }
    }

    pub fn with_gamepads(
        buttons: &ButtonInput<GamepadButton>,
        controller: &Controller,
        controller_mapping: ControllerMapping,
    ) -> Self {
        let mut inputs = Self::new();
        for gamepad in &controller.gamepads {
            inputs |= Self::with_gamepad(buttons, *gamepad, controller_mapping);
        }
        inputs
    }

    fn with_gamepad(
        buttons: &ButtonInput<GamepadButton>,
        gamepad: Gamepad,
        controller_mapping: ControllerMapping,
    ) -> Self {
        match controller_mapping {
            ControllerMapping::MappingA => Self::with_gamepad_mapping_a(buttons, gamepad),
            ControllerMapping::MappingB => Self::with_gamepad_mapping_b(buttons, gamepad),
        }
    }

    fn with_gamepad_mapping_a(buttons: &ButtonInput<GamepadButton>, gamepad: Gamepad) -> Self {
        Self {
            up: Self::from_gamepad_button(GamepadButtonType::DPadUp, buttons, gamepad),
            down: Self::from_gamepad_button(GamepadButtonType::DPadDown, buttons, gamepad),
            left: Self::from_gamepad_button(GamepadButtonType::DPadLeft, buttons, gamepad),
            right: Self::from_gamepad_button(GamepadButtonType::DPadRight, buttons, gamepad),
            a: Self::from_gamepad_button(GamepadButtonType::East, buttons, gamepad),
            b: Self::from_gamepad_button(GamepadButtonType::South, buttons, gamepad),
            start: Self::from_gamepad_button(GamepadButtonType::Start, buttons, gamepad),
            select: Self::from_gamepad_button(GamepadButtonType::Select, buttons, gamepad),
            soft_reset: buttons.pressed(Self::gamepad_button(gamepad, GamepadButtonType::Select))
                && buttons.pressed(Self::gamepad_button(gamepad, GamepadButtonType::Start))
                && buttons.pressed(Self::gamepad_button(gamepad, GamepadButtonType::East))
                && buttons.pressed(Self::gamepad_button(gamepad, GamepadButtonType::South)),
        }
    }

    fn with_gamepad_mapping_b(buttons: &ButtonInput<GamepadButton>, gamepad: Gamepad) -> Self {
        Self {
            up: Self::from_gamepad_button(GamepadButtonType::DPadUp, buttons, gamepad),
            down: Self::from_gamepad_button(GamepadButtonType::DPadDown, buttons, gamepad),
            left: Self::from_gamepad_button(GamepadButtonType::DPadLeft, buttons, gamepad),
            right: Self::from_gamepad_button(GamepadButtonType::DPadRight, buttons, gamepad),
            a: Self::from_gamepad_button(GamepadButtonType::South, buttons, gamepad),
            b: Self::from_gamepad_button(GamepadButtonType::West, buttons, gamepad),
            start: Self::from_gamepad_button(GamepadButtonType::Start, buttons, gamepad),
            select: Self::from_gamepad_button(GamepadButtonType::Select, buttons, gamepad),
            soft_reset: buttons.pressed(Self::gamepad_button(gamepad, GamepadButtonType::Select))
                && buttons.pressed(Self::gamepad_button(gamepad, GamepadButtonType::Start))
                && buttons.pressed(Self::gamepad_button(gamepad, GamepadButtonType::South))
                && buttons.pressed(Self::gamepad_button(gamepad, GamepadButtonType::West)),
        }
    }

    fn from_gamepad_button(
        button_type: GamepadButtonType,
        buttons: &ButtonInput<GamepadButton>,
        gamepad: Gamepad,
    ) -> PlayerInput {
        PlayerInput {
            just_pressed: buttons.just_pressed(Self::gamepad_button(gamepad, button_type)),
            pressed: buttons.pressed(Self::gamepad_button(gamepad, button_type)),
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
            up: self.up | rhs.up,
            down: self.down | rhs.down,
            left: self.left | rhs.left,
            right: self.right | rhs.right,
            a: self.a | rhs.a,
            b: self.b | rhs.b,
            start: self.start | rhs.start,
            select: self.select | rhs.select,
            soft_reset: self.soft_reset | rhs.soft_reset,
        }
    }
}
