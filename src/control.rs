#[derive(PartialEq)]
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
}

pub trait Control {
    fn control(&mut self);
}
