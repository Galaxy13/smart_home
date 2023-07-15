use crate::helper_traits::{Info, Report};

#[derive(PartialEq, Debug)]
pub enum PowerState {
    On,
    Off,
}

impl PowerState {
    pub fn state_name(&self) -> &str {
        match self {
            PowerState::On => "On",
            PowerState::Off => "Off",
        }
    }
}

pub trait PowerControl {
    fn power_change(&mut self);
    fn current_state(&self) -> &str;
}

pub trait DeviceInterface: PowerControl + Info {
}

impl Report for &dyn DeviceInterface {
    fn string_report(&self) -> String {
        let mut report = String::from("\nDevice name: ");
        report.push_str(self.get_name().as_str());
        report.push_str(format!("\nCurrent state: {}",  self.current_state()).as_str());
        for (key, value) in self.sensor_info(){
            report.push('\n');
            report.push_str(&format!("{} -> {}", key, value))
        }
        report
    }
}
