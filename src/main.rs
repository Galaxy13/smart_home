use std::io;
use smart_home::{power_plug, thermometer, control};
use control::Control;
use power_plug::PowerPlug;
use thermometer::Thermometer;

fn main() {
    let mut plug: PowerPlug = PowerPlug::new();
    let mut tr: Thermometer = Thermometer::new();
    loop {
        print!("\x1B[2J\x1B[1;1H");
        println!("1 - Power Plug: {}", &plug.get_state());
        println!("2 - Thermometer: {}", &tr.get_state());
        println!("Choose device: 1-2\nExit - 3");
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let command = command.trim();
        match command {
            "1" => plug.control(),
            "2" => tr.control(),
            "3" => break,
            _ => continue
        };
    }
}
