use crate::control::{Control, PowerControl, PowerState};
use std::io;

pub struct PowerPlug {
    power_state: PowerState,
    energy_power: f64,
}

impl Default for PowerPlug {
    fn default() -> Self {
        PowerPlug::new()
    }
}

impl PowerPlug {
    pub fn new() -> Self {
        PowerPlug {
            power_state: PowerState::Off,
            energy_power: 0.0,
        }
    }
    pub fn current_consumption(&self) -> &f64 {
        &self.energy_power
    }
    pub fn get_state(&self) -> &str {
        self.power_state.state_name()
    }
}

impl PowerControl for PowerPlug {
    fn power_change(&mut self) {
        if self.get_state() == "On" {
            self.power_state = PowerState::Off
        } else {
            self.power_state = PowerState::On
        }
    }
}

impl Control for PowerPlug {
    fn control(&mut self) {
        loop {
            print!("\x1B[2J\x1B[1;1H");
            println!("Plug state: {}", self.get_state());
            match self.power_state {
                PowerState::On => {
                    println!("Current consumption: {}", self.current_consumption());
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
