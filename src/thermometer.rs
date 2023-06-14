use crate::control::{Control, PowerControl, PowerState};

pub struct Thermometer {
    power_state: PowerState,
    temperature: f64,
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
        if self.get_state() == "On"{
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
            println!("Thermometer state: {}\nCurrent temperature: {}",
                     self.get_state(), self.current_temperature());
            println!("Choose action: \n 1: Turn ON/Off\n 2: Main Menu");

        }
    }
}
