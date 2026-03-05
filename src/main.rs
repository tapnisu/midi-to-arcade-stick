use midir::MidiInput;
use midly::live::LiveEvent;
use vigem_client::TargetId;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let midi_in = MidiInput::new("midi_listener")?;
    let in_ports = midi_in.ports();

    let client = vigem_client::Client::connect().unwrap();
    let mut target = vigem_client::Xbox360Wired::new(client, TargetId::XBOX360_WIRED);

    target.plugin().unwrap();
    target.wait_ready().unwrap();

    if in_ports.is_empty() {
        unimplemented!()
    }

    let port = &in_ports[0];
    let port_name = midi_in.port_name(port)?;

    let mut gamepad_pressed = vigem_client::XGamepad {
        buttons: vigem_client::XButtons!(A),
        ..Default::default()
    };

    let _conn = midi_in.connect(
        port,
        &port_name,
        move |_timestamp, data, _| {
            if let Ok(event) = LiveEvent::parse(data) {
                match event {
                    LiveEvent::Midi {
                        channel: _,
                        message,
                    } => match message {
                        midly::MidiMessage::NoteOn { key, vel } => {
                            println!("Key Pressed:  {} (Velocity: {})", key, vel);
                        }
                        midly::MidiMessage::NoteOff { key, .. } => {
                            println!("Key Released: {}", key);
                        }
                        midly::MidiMessage::PitchBend { bend } => {
                            gamepad_pressed.thumb_lx = bend.as_int() * 4;
                            let _ = target.update(&gamepad_pressed);
                        }
                        midly::MidiMessage::Controller { controller, value } => {
                            if controller.as_int() == 1 {
                                gamepad_pressed.thumb_ly = value.as_int() as i16 * -256 + -1;
                                let _ = target.update(&gamepad_pressed);
                            } else {
                                println!("Control #{} value: {}", controller, value);
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        },
        (),
    )?;

    Ok(())
}
