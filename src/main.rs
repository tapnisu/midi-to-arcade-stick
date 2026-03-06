use midi_to_arcade_stick::{Gamepad, GamepadButton, GamepadThumb, VirtualGamepad};
use midir::MidiInput;
use midly::{live::LiveEvent, num::u7};

fn key_to_button(key: u7) -> Option<GamepadButton> {
    match key.as_int() {
        64 => Some(GamepadButton::DpadUp),
        65 => Some(GamepadButton::X),
        66 => Some(GamepadButton::LT),
        67 => Some(GamepadButton::A),
        68 => Some(GamepadButton::RT),
        69 => Some(GamepadButton::B),
        71 => Some(GamepadButton::Y),
        _ => None,
    }
}

fn handle_midi_input(data: &[u8], gamepad: &mut Gamepad) {
    if let Ok(event) = LiveEvent::parse(data)
        && let LiveEvent::Midi { message, .. } = event
    {
        match message {
            midly::MidiMessage::NoteOn { key, vel } => {
                if let Some(button) = key_to_button(key) {
                    gamepad.press_button(&button, vel.as_int().saturating_mul(2));
                    gamepad.update();
                }
            }
            midly::MidiMessage::NoteOff { key, .. } => {
                if let Some(button) = key_to_button(key) {
                    gamepad.release_button(&button);
                    gamepad.update();
                }
            }
            midly::MidiMessage::PitchBend { bend } => {
                gamepad.update_axis(&GamepadThumb::ThumbLX, bend.as_int().saturating_mul(4));
                gamepad.update();
            }
            midly::MidiMessage::Controller { controller, value } => {
                if controller.as_int() == 1 {
                    let value = (value.as_int() as i16).saturating_mul(-256);
                    gamepad.update_axis(&GamepadThumb::ThumbLY, value);
                    gamepad.update();
                }
            }
            _ => {}
        }
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

    let mut gamepad = Gamepad::new()?;

    let _conn = midi_in.connect(
        port,
        &port_name,
        move |_timestamp, data, _| {
            handle_midi_input(data, &mut gamepad);
        },
        (),
    )?;

    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
