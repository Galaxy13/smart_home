pub mod control;
mod devices;
pub mod helper_traits;
mod power_plug;
mod server;

use crate::helper_traits::Info;
use helper_traits::Report;
use crate::power_plug::PowerPlug;

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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn creating_home() {
//         let home_name = String::from("test_home");
//         Home::new(home_name);
//     }
//     #[test]
//     fn check_room_name() {
//         let home_name = String::from("test_home");
//         let smarthome = Home::new(home_name);
//         assert_eq!(&String::from("test_home"), smarthome.home_name());
//     }
//     #[test]
//     #[should_panic]
//     fn check_room_name_err() {
//         let home_name = String::from("test_home");
//         let smarthome = Home::new(home_name);
//         assert_eq!(&String::from("fail"), smarthome.home_name())
//     }
//     #[test]
//     fn room_names_in_home_test() {
//         let home_name = String::from("test_home");
//         let mut smarthome = Home::new(home_name);
//         smarthome.create_room("Bedroom");
//         smarthome.create_room("Kitchen");
//         let mut room_names = smarthome.get_rooms_names().unwrap_or(Vec::new());
//         room_names.sort();
//         let mut expected_names = vec![String::from("Kitchen"), String::from("Bedroom")];
//         expected_names.sort();
//         assert_eq!(room_names, expected_names)
//     }
//     #[test]
//     fn unique_room_name() {
//         let home_name = String::from("test_home");
//         let mut smarthome = Home::new(home_name);
//         smarthome.create_room("Test");
//         smarthome.create_room("Test2");
//         smarthome.create_room("Test");
//         let mut room_names = smarthome.get_rooms_names().unwrap_or(Vec::new());
//         let room_list = smarthome.get_rooms_list().unwrap_or(Vec::new());
//         room_names.sort();
//         let mut expected_names = vec![String::from("Test"), String::from("Test2")];
//         expected_names.sort();
//         assert_eq!(room_names, expected_names);
//         assert_eq!(room_list.len(), 2)
//     }
//     #[test]
//     fn empty_rooms_list_test() {
//         let home_name = String::from("test_home");
//         let smarthome = Home::new(home_name);
//         assert_eq!(smarthome.get_rooms_names().unwrap_or_default().len(), 0)
//     }
//     #[test]
//     fn check_room_list() {
//         let home_name = String::from("test_home");
//         let mut smarthome = Home::new(home_name);
//         smarthome.create_room("Test");
//         smarthome.create_room("Test2");
//         assert_eq!(smarthome.get_rooms_list().unwrap_or_default().len(), 2)
//     }
//     #[test]
//     fn report_test() {
//         let home_name = String::from("test_home");
//         let mut smarthome = Home::new(home_name);
//         smarthome.create_room("Test");
//         smarthome.create_room("Test2");
//         assert_eq!(
//             report(&smarthome),
//             String::from(
//                 "\nSmart home name: test_home\n\
//                                                           Current rooms number: 2\n\
//                                                           Test\n\
//                                                           Test2\n"
//             )
//         )
//     }
//     #[test]
//     fn report_test_empty() {
//         let home_name = String::from("test_home");
//         let smarthome = Home::new(home_name);
//         assert_eq!(
//             report(&smarthome),
//             String::from(
//                 "\nSmart home name: test_home\n\
//                                                            Current rooms number: 0\n"
//             )
//         )
//     }
//     #[test]
//     fn create_new_room() {
//         let home_name = String::from("test_home");
//         let mut smarthome = Home::new(home_name);
//         smarthome.create_room("test_room");
//         smarthome.get_room_by_name("test_room");
//     }
//     #[test]
//     #[should_panic]
//     fn create_new_room_fail() {
//         let home_name = String::from("test_home");
//         let mut smarthome = Home::new(home_name);
//         smarthome.create_room("test_room");
//         smarthome.get_room_by_name("test_room_nonexs");
//     }
//     #[test]
//     fn create_device() {
//         let home_name = String::from("test_home");
//         let mut smarthome = Home::new(home_name);
//         smarthome.create_room("test_room");
//         let test_room = smarthome.get_room_by_name("test_room");
//         test_room.add_device(Devices::Thermometer, "test_thermo");
//         test_room.get_device("test_thermo").unwrap();
//     }
//     #[test]
//     fn create_same_device() {
//         let home_name = String::from("test_home");
//         let mut smarthome = Home::new(home_name);
//         smarthome.create_room("test_room");
//         let test_room = smarthome.get_room_by_name("test_room");
//         test_room.add_device(Devices::Thermometer, "test_thermo");
//         test_room.add_device(Devices::Thermometer, "test_thermo");
//         assert_eq!(test_room.get_devices().len(), 1)
//     }
//     #[test]
//     fn remove_room() {
//         let home_name = String::from("test_home");
//         let mut smarthome = Home::new(home_name);
//         smarthome.create_room("test_room");
//         smarthome.remove_room("test_room");
//         assert_eq!(smarthome.get_rooms_list().unwrap_or(vec![]).len(), 0);
//     }
//     #[test]
//     fn remove_room_fail() {
//         let home_name = String::from("test_home");
//         let mut smarthome = Home::new(home_name);
//         smarthome.create_room("test_room");
//         smarthome.remove_room("no_room");
//         assert_eq!(smarthome.get_rooms_list().unwrap().len(), 1);
//     }
//     #[test]
//     fn remove_device() {
//         let home_name = String::from("test_home");
//         let mut smarthome = Home::new(home_name);
//         smarthome.create_room("test_room");
//         let test_room = smarthome.get_room_by_name("test_room");
//         test_room.add_device(Devices::Thermometer, "test_thermo");
//         test_room.remove_device("test_thermo");
//         assert_eq!(test_room.get_devices().len(), 0);
//     }
//     #[test]
//     fn remove_device_fail() {
//         let home_name = String::from("test_home");
//         let mut smarthome = Home::new(home_name);
//         smarthome.create_room("test_room");
//         let test_room = smarthome.get_room_by_name("test_room");
//         test_room.add_device(Devices::Thermometer, "test_thermo");
//         test_room.remove_device("test_thermo2");
//         assert_eq!(
//             test_room.get_device("test_thermo").unwrap().get_name(),
//             "test_thermo".to_string()
//         )
//     }
// }
