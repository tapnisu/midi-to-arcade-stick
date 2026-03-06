use midi_to_arcade_stick::{Gamepad, MidiController};
use midir::MidiInput;

const CONFIG_PATH: &str = "keybinds.cfg";

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

    let gamepad = Gamepad::new()?;
    let mut midi_controller = MidiController::new(gamepad, CONFIG_PATH);

    let _conn = midi_in.connect(
        port,
        &port_name,
        move |_timestamp, data, _| {
            midi_controller.handle_midi_input(data);
        },
        (),
    )?;

    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
