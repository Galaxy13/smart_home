use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use smarthome_lib::create_powerplug;
use smarthome_lib::control::{PowerControl, PowerState};
use smarthome_lib::helper_traits::Info;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:23015").unwrap();
    let mut power_plug = create_powerplug("Power_Plug1");

    for stream in listener.incoming(){
        if stream.is_err(){
            continue
        }
        let stream = stream.unwrap();
        let sender = stream.peer_addr();
        println!("Received command from {:?}", sender);
        let command = handle_connection(stream);
        let res = command.trim_matches(char::from(0));
        let command: Vec<&str> = res.split_whitespace().collect();
        if !command.is_empty(){
            let base_command = command[0];
            match base_command{
                "state" => if command.get(1).is_some(){
                    let argument = &command[1];
                    match argument as &str {
                        "on" => {power_plug.power_change(PowerState::On);
                        println!("Power is On")},
                        "off" => {power_plug.power_change(PowerState::Off);
                        println!("Power if Off")},
                        _ => println!("Wrong argument!")
                    }
                },
                "get" => if command.get(1).is_some(){
                    let argument = &command[1];
                    match argument as &str {
                        "state" => println!("Current state: {}", power_plug.get_state()),
                        "consumption" => println!("Current power consumption: {}", power_plug.current_consumption()),
                        "name" => println!("Plug name: {}", power_plug.get_name()),
                        _ => println!("Wrong argument")
                    }
                },
                _ => println!("Wrong command")
            }
        }
        println!("Command from {:?} completed", sender)
    }
}

fn handle_connection(mut tcp_stream: TcpStream) -> String {
    let mut data = [0_u8; 20];
    match tcp_stream.read(&mut data) {
        Ok(size) => {
            tcp_stream.write_all(&data[0..size]).unwrap();
        },
        Err(e) => {
            println!("Error {} occurred", e);
            tcp_stream.shutdown(Shutdown::Both).unwrap();
        },
    }
    String::from_utf8_lossy(&data).to_string()
}
