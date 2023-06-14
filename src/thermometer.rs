use crate::control::{Control, PowerControl, PowerState};
use std::io;

pub struct Thermometer {
    power_state: PowerState,
    temperature: f64,
}

impl Default for Thermometer {
    fn default() -> Self {
        Thermometer::new()
    }
}

impl Thermometer {
    pub fn new() -> Self {
        Thermometer {
            power_state: PowerState::Off,
            temperature: 20.0,
        }
    }
    pub fn current_temperature(&self) -> f64 {
        self.temperature
    }
    pub fn get_state(&self) -> &str {
        self.power_state.state_name()
    }
}

impl PowerControl for Thermometer {
    fn power_change(&mut self) {
        if self.get_state() == "On" {
            self.power_state = PowerState::Off;
        } else {
            self.power_state = PowerState::On
        }
        println!("Thermometer is turned {}", self.get_state())
    }
}

impl Control for Thermometer {
    fn control(&mut self) {
        loop {
            print!("\x1B[2J\x1B[1;1H");
            println!("Thermometer state: {}", self.get_state());
            match self.power_state {
                PowerState::On => {
                    println!("Current temperature: {}", self.current_temperature())
                }
                PowerState::Off => (),
            }
            println!("Choose action: \n 1: Turn ON/Off\n 2: Main Menu");
            let mut command = String::new();
            io::stdin().read_line(&mut command).unwrap();
            let command = command.trim();
            match command {
                "1" => self.power_change(),
                "2" => break,
                _ => continue,
            }
        }
    }
}
