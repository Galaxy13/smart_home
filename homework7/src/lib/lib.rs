pub mod control;
mod devices;
pub mod helper_traits;
mod power_plug;
mod server;

use crate::helper_traits::Info;
use crate::power_plug::PowerPlug;
use helper_traits::Report;

pub fn report<T: Report>(entity: &T) -> String {
    entity.string_report()
}

pub fn create_powerplug(device_name: &str) -> PowerPlug {
    PowerPlug::new(device_name.to_string())
}

pub enum Devices {
    Thermometer,
    PowerPlug,
}
