use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
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
        if !command.is_empty(){
            let base_command = &command[0] as &str;
            match base_command{
                "state" => if command.get(1).is_some(){
                    let argument = &command[1];
                    match argument as &str {
                        "on" => power_plug.power_change(PowerState::On),
                        "off" => power_plug.power_change(PowerState::Off),
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

fn handle_connection(mut tcp_stream: TcpStream) -> Vec<String> {
    let buf_reader = BufReader::new(&mut tcp_stream);
    let command: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    command
}
