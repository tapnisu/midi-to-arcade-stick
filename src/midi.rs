use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use midly::live::LiveEvent;

use crate::{Gamepad, GamepadButton, GamepadThumb, VirtualGamepad};

pub struct MidiController {
    gamepad: Gamepad,
    binds: HashMap<u8, GamepadButton>,
}

impl MidiController {
    pub fn new(gamepad: Gamepad, config_path: &str) -> Self {
        let binds = Self::load_keybinds(config_path);
        Self { gamepad, binds }
    }

    fn load_keybinds(path: &str) -> HashMap<u8, GamepadButton> {
        let mut binds = HashMap::new();

        let file = match File::open(path) {
            Ok(f) => f,
            Err(_) => {
                println!("{} not found", path);
                return binds;
            }
        };

        for line in BufReader::new(file).lines().flatten() {
            let parts: Vec<&str> = line.split_whitespace().collect();

            if parts.len() >= 3 && parts[0] == "bind" {
                let midi_id = parts[1].parse::<u8>().unwrap_or(0);
                let button = match parts[2].to_lowercase().as_str() {
                    "up" => Some(GamepadButton::DpadUp),
                    "down" => Some(GamepadButton::DpadDown),
                    "left" => Some(GamepadButton::DpadLeft),
                    "right" => Some(GamepadButton::DpadRight),
                    "a" => Some(GamepadButton::A),
                    "b" => Some(GamepadButton::B),
                    "x" => Some(GamepadButton::X),
                    "y" => Some(GamepadButton::Y),
                    "lb" => Some(GamepadButton::LB),
                    "rb" => Some(GamepadButton::RB),
                    "lt" => Some(GamepadButton::LT),
                    "rt" => Some(GamepadButton::RT),
                    "l3" => Some(GamepadButton::L3),
                    "r3" => Some(GamepadButton::R3),
                    "start" => Some(GamepadButton::Start),
                    "guide" | "back" => Some(GamepadButton::Guide),
                    _ => None,
                };

                if let Some(btn) = button {
                    binds.insert(midi_id, btn);
                }
            }
        }
        binds
    }

    pub fn handle_midi_input(&mut self, data: &[u8]) {
        if let Ok(event) = LiveEvent::parse(data)
            && let LiveEvent::Midi { message, .. } = event
        {
            match message {
                midly::MidiMessage::NoteOn { key, vel } => {
                    if let Some(button) = self.binds.get(&key.as_int()) {
                        self.gamepad
                            .press_button(button, vel.as_int().saturating_mul(2));
                        self.gamepad.update();
                    }
                }
                midly::MidiMessage::NoteOff { key, .. } => {
                    if let Some(button) = self.binds.get(&key.as_int()) {
                        self.gamepad.release_button(button);
                        self.gamepad.update();
                    }
                }
                midly::MidiMessage::PitchBend { bend } => {
                    self.gamepad
                        .update_axis(&GamepadThumb::ThumbLX, bend.as_int().saturating_mul(4));
                    self.gamepad.update();
                }
                midly::MidiMessage::Controller { controller, value } => {
                    if controller.as_int() == 1 {
                        let val = (value.as_int() as i16).saturating_mul(-256);
                        self.gamepad.update_axis(&GamepadThumb::ThumbLY, val);
                        self.gamepad.update();
                    }
                }
                _ => {}
            }
        }
    }
}
