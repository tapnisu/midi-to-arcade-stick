use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use midly::{MidiMessage, live::LiveEvent, num::u7};

use crate::{Gamepad, GamepadButton, GamepadThumb, VirtualGamepad};

pub struct MidiController {
    gamepad: Gamepad,
    config: MidiControllerConfig,
}

#[derive(Default)]
pub struct MidiControllerConfig {
    binds: HashMap<u8, GamepadButton>,
    enable_rt_value: bool,
    enable_lt_value: bool,
}

impl MidiController {
    pub fn new(gamepad: Gamepad, config_path: &str) -> Self {
        let config = Self::load_keybinds(config_path);
        Self { gamepad, config }
    }

    fn load_keybinds(path: &str) -> MidiControllerConfig {
        let mut config = MidiControllerConfig::default();

        let file = match File::open(path) {
            Ok(f) => f,
            Err(_) => {
                println!("{} not found", path);
                return config;
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
                    config.binds.insert(midi_id, btn);
                }
            }

            if parts.len() >= 2 && (parts[0] == "enable_rt_value") {
                config.enable_rt_value = match parts[1] {
                    "true" => true,
                    "false" => false,
                    _ => continue,
                };
            }

            if parts.len() >= 2 && (parts[0] == "enable_lt_value") {
                config.enable_lt_value = match parts[1] {
                    "true" => true,
                    "false" => false,
                    _ => continue,
                };
            }
        }
        config
    }

    pub fn handle_midi_input(&mut self, data: &[u8]) {
        if let Ok(event) = LiveEvent::parse(data)
            && let LiveEvent::Midi { message, .. } = event
        {
            match message {
                MidiMessage::NoteOn { key, vel } => {
                    println!("key pressed {}", key.as_int());
                    if let Some(button) = self.config.binds.get(&key.as_int()) {
                        let enable_value = match button {
                            GamepadButton::LT => self.config.enable_lt_value,
                            GamepadButton::RT => self.config.enable_rt_value,
                            _ => false,
                        };
                        let enabled_vel = if enable_value { vel } else { u7::max_value() };

                        self.gamepad
                            .press_button(button, enabled_vel.as_int().saturating_mul(2));
                        self.gamepad.update();
                    }
                }
                MidiMessage::NoteOff { key, .. } => {
                    if let Some(button) = self.config.binds.get(&key.as_int()) {
                        self.gamepad.release_button(button);
                        self.gamepad.update();
                    }
                }
                MidiMessage::PitchBend { bend } => {
                    // self.gamepad
                    //     .update_axis(&GamepadThumb::ThumbLX, bend.as_int().saturating_mul(4));
                    self.gamepad
                        .update_axis(&GamepadThumb::ThumbLY, bend.as_int().saturating_mul(4));
                    self.gamepad.update();
                }
                midly::MidiMessage::Controller { controller, value } => {
                    match controller.as_int() {
                        1 => {
                            // let val = (value.as_int() as i16).saturating_mul(-256);
                            // self.gamepad.update_axis(&GamepadThumb::ThumbLY, val);
                            // self.gamepad.update();

                            // Modwheel
                            let val = (value.as_int() as i16).saturating_mul(256);
                            self.gamepad.update_axis(&GamepadThumb::ThumbLX, val);
                            self.gamepad.update();
                        }

                        11 => {
                            // Expression
                            let val = (127 - value.as_int() as i16).saturating_mul(-256);
                            self.gamepad.update_axis(&GamepadThumb::ThumbLX, val);
                            self.gamepad.update();
                        }
                        _ => {}
                    }
                }

                _ => {}
            }
        }
    }
}
