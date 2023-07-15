use smarthome_lib::{create_smarthome, report};
use smarthome_lib::Devices::{Thermometer, PowerPlug};
use smarthome_lib::home::Home;

fn main(){
    // Creating main Home instance
    // Acceptable name: String
    let home_name = String::from("Example Home");
    let mut smart_home = Home::new(home_name);
    // or
    let mut _smart_home2 = create_smarthome("Unused Home");

    // Add a few rooms to smart_home
    // Acceptable name: &str
    smart_home.create_room("Room 1");
    smart_home.create_room("Room 2");

    // Adding a new devices to Room 1 and Room 2
    smart_home.create_device("Room 1", PowerPlug, "powerplug_1");
    smart_home.create_device("Room 2", Thermometer, "thermometer_1");

    // Using gen. function ``report<T: Report>`` report from Home, Room and Devices can be obtained
    print!("{}", report(&smart_home));
    let room_1 = smart_home.get_room_by_name("Room 1");
    print!("{}", report(&room_1));
    let room_2 = smart_home.get_room_by_name("Room 2");
    print!("{}", report(&room_2));
    let rooms = smart_home.get_rooms_list();
    for room in rooms{
        let devices = room.get_devices().values();
        for device in devices{
            print!("{}", report(&device.as_ref()))
        }
    }
}