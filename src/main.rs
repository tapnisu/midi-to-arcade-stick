use midir::MidiInput;
use midly::live::LiveEvent;
use vigem_client::{TargetId, XButtons, XGamepad};

fn key_to_button(key: u8) -> Option<u16> {
    match key {
        64 => Some(XButtons::UP),
        65 => Some(XButtons::X),
        67 => Some(XButtons::A),
        69 => Some(XButtons::B),
        71 => Some(XButtons::Y),
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
    let mut target = vigem_client::Xbox360Wired::new(client, TargetId::XBOX360_WIRED);

    target.plugin().unwrap();
    target.wait_ready().unwrap();

    let mut gamepad_pressed = XGamepad::default();

    let _conn = midi_in.connect(
        port,
        &port_name,
        move |_timestamp, data, _| {
            if let Ok(event) = LiveEvent::parse(data) {
                match event {
                    LiveEvent::Midi { message, .. } => match message {
                        midly::MidiMessage::NoteOn { key, vel: _ } => {
                            if let Some(button_bit) = key_to_button(key.as_int()) {
                                gamepad_pressed.buttons.raw |= button_bit;
                                let _ = target.update(&gamepad_pressed);
                            }
                            println!("Key pressed: {}", key);
                        }
                        midly::MidiMessage::NoteOff { key, .. } => {
                            if let Some(button_bit) = key_to_button(key.as_int()) {
                                gamepad_pressed.buttons.raw &= !button_bit;
                                let _ = target.update(&gamepad_pressed);
                            }
                            println!("Key released: {}", key);
                        }
                        midly::MidiMessage::PitchBend { bend } => {
                            gamepad_pressed.thumb_lx = bend.as_int() as i16 * 4;
                            let _ = target.update(&gamepad_pressed);
                            println!("Pitch changed: {}", bend.as_int());
                        }
                        midly::MidiMessage::Controller { controller, value } => {
                            if controller.as_int() == 1 {
                                gamepad_pressed.thumb_ly = value.as_int() as i16 * -256 + -1;
                                let _ = target.update(&gamepad_pressed);
                            }
                            println!("Mod changed: {}", value.as_int());
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        },
        (),
    )?;

    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Ok(())
}
