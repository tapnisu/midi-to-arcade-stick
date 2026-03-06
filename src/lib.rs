mod midi;

pub use midi::MidiController;

#[cfg(target_os = "windows")]
mod windows;

pub enum GamepadButton {
    DpadUp,
    DpadDown,
    DpadLeft,
    DpadRight,
    A,
    B,
    X,
    Y,
    LB,
    RB,
    LT,
    RT,
    L3,
    R3,
    Start,
    Guide,
}

pub enum GamepadThumb {
    ThumbLX,
    ThumbLY,
    ThumbRX,
    ThumbRY,
}

pub trait VirtualGamepad {
    fn press_button(&mut self, button: &GamepadButton, velocity: u8);
    fn release_button(&mut self, button: &GamepadButton);
    fn update_axis(&mut self, axis: &GamepadThumb, value: i16);
    fn update(&mut self);
}

pub use windows::Gamepad;
