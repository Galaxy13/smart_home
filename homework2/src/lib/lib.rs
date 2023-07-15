mod control;
mod power_plug;
mod thermometer;
pub mod home;
mod room;
mod devices;
mod helper_traits;
use helper_traits::Report;
use crate::home::Home;
use crate::helper_traits::Info;

pub fn report<T: Report>(entity: &T) -> String{
    entity.string_report()
}

pub fn create_smarthome(home_name: &str) -> Home {
    Home::new(String::from(home_name))
}

pub enum Devices{
    Thermometer,
    PowerPlug
}



