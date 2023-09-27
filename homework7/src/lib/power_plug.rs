use crate::control::{DeviceInterface, PowerControl, PowerState};
use crate::{Devices, Info};
use std::collections::HashMap;

pub struct PowerPlug {
    name: String,
    device_type: Devices,
    power_state: PowerState,
    energy_power: f64,
}

impl Default for PowerPlug {
    fn default() -> Self {
        PowerPlug {
            name: String::from("Untitled"),
            device_type: Devices::PowerPlug,
            power_state: PowerState::Off,
            energy_power: 0.0,
        }
    }
}

impl PowerPlug {
    pub fn new(device_name: String) -> Self {
        PowerPlug {
            name: device_name,
            device_type: Devices::PowerPlug,
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
    fn power_change(&mut self, new_state: PowerState) {
        match new_state {
            PowerState::On => self.power_state = PowerState::On,
            PowerState::Off => self.power_state = PowerState::Off
        }
    }
    fn current_state(&self) -> &str {
        self.get_state()
    }
}

impl Info for PowerPlug {
    fn get_name(&self) -> String {
        String::from(&self.name)
    }
    fn device_type(&self) -> &Devices {
        &self.device_type
    }
    fn sensor_info(&self) -> HashMap<String, String> {
        let mut info = HashMap::new();
        info.insert("Consumption".to_string(), self.energy_power.to_string());
        info
    }
}

impl DeviceInterface for PowerPlug {}
