use smarthome_lib::home::Home;
use smarthome_lib::Devices::Thermometer;
use smarthome_lib::{create_smarthome, report};

fn main() {
    let mut smart_home: Home = create_smarthome("House 1");
    smart_home.create_room("Kitchen");
    smart_home.create_room("Bedroom");
    let mut smart_home2: Home = create_smarthome("House 2");
    smart_home2.create_room("Kitchen");
    smart_home2.create_room("Bathroom");
    print!("{}", report(&smart_home));
    print!("{}", report(&smart_home2));

    smart_home.create_device("Kitchen", Thermometer, "thermo_2");
    let kitchen = smart_home.get_room_by_name("Kitchen");
    print!("{}", report(&kitchen));
    kitchen.add_device(Thermometer, "thermo_1");
    let device = kitchen.get_device("thermo_1").expect("No device found");
    print!("{}", report(&device));
}
