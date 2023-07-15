use std::collections::HashMap;
use crate::Devices;

pub trait Info {
    fn get_name(&self) -> String;
    fn device_type(&self) -> &Devices;
    fn sensor_info(&self) -> HashMap<String, String>;
}

pub trait Report{
    fn string_report(&self) -> String;
}

