use vigem_client::{Client, TargetId, XButtons, XGamepad, XTarget, Xbox360Wired};

use crate::{GamepadButton, GamepadThumb, VirtualGamepad};

pub struct Gamepad {
    xtarget: XTarget,
    xgamepad: XGamepad,
}

impl Gamepad {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let client = Client::connect()?;
        let mut xtarget = Xbox360Wired::new(client, TargetId::XBOX360_WIRED);
        xtarget.plugin()?;
        xtarget.wait_ready()?;
        Ok(Self {
            xtarget,
            xgamepad: XGamepad::default(),
        })
    }
}

impl VirtualGamepad for Gamepad {
    fn press_button(&mut self, button: &GamepadButton, velocity: u8) {
        match button {
            GamepadButton::DpadUp => self.xgamepad.buttons.raw |= XButtons::UP,
            GamepadButton::DpadDown => self.xgamepad.buttons.raw |= XButtons::DOWN,
            GamepadButton::DpadLeft => self.xgamepad.buttons.raw |= XButtons::LEFT,
            GamepadButton::DpadRight => self.xgamepad.buttons.raw |= XButtons::RIGHT,
            GamepadButton::X => self.xgamepad.buttons.raw |= XButtons::X,
            GamepadButton::A => self.xgamepad.buttons.raw |= XButtons::A,
            GamepadButton::B => self.xgamepad.buttons.raw |= XButtons::B,
            GamepadButton::Y => self.xgamepad.buttons.raw |= XButtons::Y,
            GamepadButton::LB => self.xgamepad.buttons.raw |= XButtons::LB,
            GamepadButton::RB => self.xgamepad.buttons.raw |= XButtons::RB,
            GamepadButton::LT => self.xgamepad.left_trigger = velocity,
            GamepadButton::RT => self.xgamepad.right_trigger = velocity,
            GamepadButton::L3 => self.xgamepad.buttons.raw |= XButtons::LTHUMB,
            GamepadButton::R3 => self.xgamepad.buttons.raw |= XButtons::RTHUMB,
            GamepadButton::Start => self.xgamepad.buttons.raw |= XButtons::START,
            GamepadButton::Guide => self.xgamepad.buttons.raw |= XButtons::GUIDE,
        }
    }

    fn release_button(&mut self, button: &GamepadButton) {
        match button {
            GamepadButton::DpadUp => self.xgamepad.buttons.raw &= !XButtons::UP,
            GamepadButton::DpadDown => self.xgamepad.buttons.raw &= !XButtons::DOWN,
            GamepadButton::DpadLeft => self.xgamepad.buttons.raw &= !XButtons::LEFT,
            GamepadButton::DpadRight => self.xgamepad.buttons.raw &= !XButtons::RIGHT,
            GamepadButton::X => self.xgamepad.buttons.raw &= !XButtons::X,
            GamepadButton::A => self.xgamepad.buttons.raw &= !XButtons::A,
            GamepadButton::B => self.xgamepad.buttons.raw &= !XButtons::B,
            GamepadButton::Y => self.xgamepad.buttons.raw &= !XButtons::Y,
            GamepadButton::LB => self.xgamepad.buttons.raw &= !XButtons::LB,
            GamepadButton::RB => self.xgamepad.buttons.raw &= !XButtons::RB,
            GamepadButton::LT => self.xgamepad.left_trigger = 0,
            GamepadButton::RT => self.xgamepad.right_trigger = 0,
            GamepadButton::L3 => self.xgamepad.buttons.raw &= !XButtons::LTHUMB,
            GamepadButton::R3 => self.xgamepad.buttons.raw &= !XButtons::RTHUMB,
            GamepadButton::Start => self.xgamepad.buttons.raw &= !XButtons::START,
            GamepadButton::Guide => self.xgamepad.buttons.raw &= !XButtons::GUIDE,
        }
    }

    fn update_axis(&mut self, axis: &GamepadThumb, value: i16) {
        match axis {
            GamepadThumb::ThumbLX => self.xgamepad.thumb_lx = value,
            GamepadThumb::ThumbLY => self.xgamepad.thumb_ly = value,
        }
    }

    fn update(&mut self) {
        let _ = self.xtarget.update(&self.xgamepad);
    }
}
