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

    Ok(())
}

fn handle_midi_input(data: &[u8], xtarget: &mut vigem_client::XTarget, gamepad: &mut XGamepad) {
    if let Ok(event) = LiveEvent::parse(data) {
        match event {
            LiveEvent::Midi { message, .. } => match message {
                midly::MidiMessage::NoteOn { key, vel } => {
                    if vel == 66 {
                        gamepad.left_trigger = u8::from(vel) * 2;
                    }

                    if vel == 68 {
                        gamepad.left_trigger = u8::from(vel) * 2;
                    }

                    if let Some(button_bit) = key_to_button(key.as_int()) {
                        gamepad.buttons.raw |= button_bit;
                    }

                    let _ = xtarget.update(&gamepad);
                    println!("Key pressed: {}", key);
                }
                midly::MidiMessage::NoteOff { key, .. } => {
                    if let Some(button_bit) = key_to_button(key.as_int()) {
                        gamepad.buttons.raw &= !button_bit;
                        let _ = xtarget.update(&gamepad);
                    }
                    println!("Key released: {}", key);
                }
                midly::MidiMessage::PitchBend { bend } => {
                    gamepad.thumb_lx = bend.as_int() as i16 * 4;
                    let _ = xtarget.update(&gamepad);
                    println!("Pitch changed: {}", bend.as_int());
                }
                midly::MidiMessage::Controller { controller, value } => {
                    if controller.as_int() == 1 {
                        gamepad.thumb_ly = value.as_int() as i16 * -256 + -1;
                        let _ = xtarget.update(&gamepad);
                    }
                    println!("Mod changed: {}", value.as_int());
                }
                _ => {}
            },
            _ => {}
        }
    }
}
