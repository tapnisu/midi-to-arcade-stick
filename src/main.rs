use midir::MidiInput;
use midly::{live::LiveEvent, num::u7};
use vigem_client::{TargetId, XButtons, XGamepad};

pub enum GamepadButton {
    DpadUp,
    DpadDown,
    DpadLeft,
    DpadRight,
    X,
    A,
    B,
    Y,
    LeftTrigger,
    RightTrigger,
    Start,
    Guide,
}

pub enum GamepadThumb {
    ThumbLX,
    ThumbLY,
}

pub trait GamepadButtonUpdate {
    fn press(&self, gamepad: &mut XGamepad, xtarget: &mut vigem_client::XTarget, velocity: u7);
    fn release(&self, gamepad: &mut XGamepad, xtarget: &mut vigem_client::XTarget);
}

pub trait GamepadThumpUpdate {
    fn apply(&self, gamepad: &mut XGamepad, xtarget: &mut vigem_client::XTarget, value: i16);
}

impl GamepadButtonUpdate for GamepadButton {
    fn press(&self, gamepad: &mut XGamepad, xtarget: &mut vigem_client::XTarget, vel: u7) {
        let vel_u8 = vel.as_int();
        match self {
            GamepadButton::DpadUp => gamepad.buttons.raw |= XButtons::UP,
            GamepadButton::DpadDown => gamepad.buttons.raw |= XButtons::DOWN,
            GamepadButton::DpadLeft => gamepad.buttons.raw |= XButtons::LEFT,
            GamepadButton::DpadRight => gamepad.buttons.raw |= XButtons::RIGHT,
            GamepadButton::X => gamepad.buttons.raw |= XButtons::X,
            GamepadButton::A => gamepad.buttons.raw |= XButtons::A,
            GamepadButton::B => gamepad.buttons.raw |= XButtons::B,
            GamepadButton::Y => gamepad.buttons.raw |= XButtons::Y,
            GamepadButton::LeftTrigger => gamepad.left_trigger = vel_u8.saturating_mul(2),
            GamepadButton::RightTrigger => gamepad.right_trigger = vel_u8.saturating_mul(2),
            GamepadButton::Start => gamepad.buttons.raw |= XButtons::START,
            GamepadButton::Guide => gamepad.buttons.raw |= XButtons::GUIDE,
        }
        let _ = xtarget.update(gamepad);
    }

    fn release(&self, gamepad: &mut XGamepad, xtarget: &mut vigem_client::XTarget) {
        match self {
            GamepadButton::DpadUp => gamepad.buttons.raw &= !XButtons::UP,
            GamepadButton::DpadDown => gamepad.buttons.raw &= XButtons::DOWN,
            GamepadButton::DpadLeft => gamepad.buttons.raw &= XButtons::LEFT,
            GamepadButton::DpadRight => gamepad.buttons.raw &= XButtons::RIGHT,
            GamepadButton::X => gamepad.buttons.raw &= !XButtons::X,
            GamepadButton::A => gamepad.buttons.raw &= !XButtons::A,
            GamepadButton::B => gamepad.buttons.raw &= !XButtons::B,
            GamepadButton::Y => gamepad.buttons.raw &= !XButtons::Y,
            GamepadButton::LeftTrigger => gamepad.left_trigger = 0,
            GamepadButton::RightTrigger => gamepad.right_trigger = 0,
            GamepadButton::Start => gamepad.buttons.raw &= XButtons::START,
            GamepadButton::Guide => gamepad.buttons.raw &= XButtons::GUIDE,
        }
        let _ = xtarget.update(gamepad);
    }
}

impl GamepadThumpUpdate for GamepadThumb {
    fn apply(&self, gamepad: &mut XGamepad, xtarget: &mut vigem_client::XTarget, value: i16) {
        match self {
            GamepadThumb::ThumbLX => {
                gamepad.thumb_lx = value.saturating_mul(4);
            }
            GamepadThumb::ThumbLY => {
                gamepad.thumb_ly = value.saturating_mul(-256);
            }
        }
        let _ = xtarget.update(gamepad);
    }
}

fn key_to_button(key: u7) -> Option<GamepadButton> {
    match key.as_int() {
        64 => Some(GamepadButton::DpadUp),
        65 => Some(GamepadButton::X),
        66 => Some(GamepadButton::LeftTrigger),
        68 => Some(GamepadButton::RightTrigger),
        67 => Some(GamepadButton::A),
        69 => Some(GamepadButton::B),
        71 => Some(GamepadButton::Y),
        _ => None,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let midi_in = MidiInput::new("midi-to-arcade-stick")?;
    let in_ports = midi_in.ports();

    if in_ports.is_empty() {
        println!("No MIDI inputs found");
        return Ok(());
    }

    let port = &in_ports[0];
    let port_name = midi_in.port_name(port)?;
    println!("Connected to MIDI: {}", port_name);

    let client = vigem_client::Client::connect().unwrap();
    let mut xtarget = vigem_client::Xbox360Wired::new(client, TargetId::XBOX360_WIRED);

    xtarget.plugin().unwrap();
    xtarget.wait_ready().unwrap();

    let mut gamepad = XGamepad::default();

    let _conn = midi_in.connect(
        port,
        &port_name,
        move |_timestamp, data, _| {
            handle_midi_input(data, &mut xtarget, &mut gamepad);
        },
        (),
    )?;

    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

fn handle_midi_input(data: &[u8], xtarget: &mut vigem_client::XTarget, gamepad: &mut XGamepad) {
    if let Ok(event) = LiveEvent::parse(data)
        && let LiveEvent::Midi { message, .. } = event
    {
        match message {
            midly::MidiMessage::NoteOn { key, vel } => {
                if let Some(button) = key_to_button(key) {
                    button.press(gamepad, xtarget, vel);
                }
            }
            midly::MidiMessage::NoteOff { key, .. } => {
                if let Some(button) = key_to_button(key) {
                    button.release(gamepad, xtarget);
                }
            }
            midly::MidiMessage::PitchBend { bend } => {
                GamepadThumb::ThumbLX.apply(gamepad, xtarget, bend.as_int());
            }
            midly::MidiMessage::Controller { controller, value } => {
                if controller.as_int() == 1 {
                    GamepadThumb::ThumbLY.apply(gamepad, xtarget, value.as_int() as i16);
                }
            }
            _ => {}
        }
    }
}
