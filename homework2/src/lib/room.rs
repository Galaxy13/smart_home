use std::collections::HashMap;
use crate::control::{DeviceInterface};
use crate::{Devices, Report};
use crate::power_plug::PowerPlug;
use crate::thermometer::Thermometer;

pub struct Room {
    room_name: String,
    devices: HashMap<String, Box<dyn DeviceInterface>>
}

impl Room {
    pub fn new(room_name: String) -> Self{
        Room{
            room_name,
            devices: HashMap::new()
        }
    }

    pub fn room_name(&self) -> &String{
        &self.room_name
    }

    pub fn get_devices(&self) -> &HashMap<String, Box<dyn DeviceInterface>> {
        &self.devices
    }

    pub fn get_device(&self, device_name: &str) -> Result<&dyn DeviceInterface, String> {
        if self.devices.contains_key(&device_name.to_string()){
        Ok(self.devices.get(device_name).unwrap().as_ref())}
        else { Err(String::from("No device contains in this room")) }
    }

    pub fn get_devices_names(&self) -> Vec<String> {
        let mut devices_t: Vec<String> = vec![];
        for (device_name, _) in self.devices.iter(){
            devices_t.push(device_name.clone());
        }
        devices_t
    }

    pub fn add_device(&mut self, device_type: Devices, device_name: &str) {
        match device_type {
            Devices::PowerPlug => self.devices.insert(device_name.to_string(), Box::new(PowerPlug::new(device_name.to_string()))),
            Devices::Thermometer => self.devices.insert(device_name.to_string(), Box::new(Thermometer::new(device_name.to_string(), 0.0)))
        };
    }
}

impl Report for &mut Room {
    fn string_report(&self) -> String {
        let mut room_report = String::from("\nRoom name: ");
        room_report.push_str(self.room_name());
        room_report.push_str("\nDevices:");
        for device_name in self.get_devices_names(){
            room_report.push('\n');
            room_report.push_str(device_name.as_str());
        }
        room_report.push('\n');
        room_report
    }
}
