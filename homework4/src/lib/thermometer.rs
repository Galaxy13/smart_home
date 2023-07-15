use std::collections::HashMap;
use crate::control::{DeviceInterface, PowerControl, PowerState};
use crate::{Devices, Info};


pub struct Thermometer {
    name: String,
    device_type: Devices,
    power_state: PowerState,
    temperature: f64,
}

impl Default for Thermometer {
    fn default() -> Self {
        Thermometer{
            name: String::from("Untitled"),
            device_type: Devices::Thermometer,
            power_state: PowerState::Off,
            temperature: 0.0
        }
    }
}

impl Thermometer {
    pub fn new(device_name: String, temperature: f64) -> Self {
        Thermometer {
            name: device_name,
            device_type: Devices::Thermometer,
            power_state: PowerState::Off,
            temperature,
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

    fn current_state(&self) -> &str {
        self.get_state()
    }
}

impl Info for Thermometer{
    fn get_name(&self) -> String {
        String::from(&self.name)
    }
    fn device_type(&self) -> &Devices {
        &self.device_type
    }
    fn sensor_info(&self) -> HashMap<String, String> {
        let mut info = HashMap::new();
        info.insert("Current temperature".to_string(), self.temperature.to_string());
        info
    }
}

impl DeviceInterface for Thermometer{ }

