use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use smarthome_lib::create_powerplug;
use smarthome_lib::control::{PowerControl, PowerState};
use smarthome_lib::helper_traits::Info;


enum AnswerCode {
    Success,
    InternalError,
}

impl AnswerCode{
    fn value(&self)  -> u8{
        match *self {
            AnswerCode::Success => 1u8,
            AnswerCode::InternalError => 100u8,
        }
    }
}

fn handle_request(code: AnswerCode, mut stream: &TcpStream, msg: &[u8]) -> Result<usize, std::io::Error> {
    let mut buf: Vec<u8> = Vec::new();
    let msg_size = msg.len() as u8;
    buf.push(code.value());
    buf.push(msg_size);
    buf.extend_from_slice(msg);
    stream.write(&buf)
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:23015").unwrap();
    let mut power_plug = create_powerplug("Power_Plug1");

    for stream in listener.incoming(){
        if stream.is_err(){
            continue
        }
        let stream = stream.unwrap();
        let sender = stream.peer_addr().unwrap();
        println!("Received command from {:?}", sender);
        let command = handle_connection(&stream);
        match command {
            [0_u8] => {
                power_plug.power_change(PowerState::On);
                if handle_request(AnswerCode::Success, &stream, &[],).is_err(){
                    continue
                }
                println!("State of plug {} is changed to On", power_plug.get_name())
            },
            [1_u8] => {
                power_plug.power_change(PowerState::Off);
                if handle_request(AnswerCode::Success, &stream, &[],).is_err(){
                    continue
                }
                println!("State of plug {} is changed to Off", power_plug.get_name())
            },
            [2_u8] => {
                let power = power_plug.current_consumption().to_be_bytes();
                if handle_request(AnswerCode::Success, &stream, &power,).is_err(){
                    continue
                }
                println!("Consumption of power plug {} is {}", power_plug.get_name(), power_plug.current_consumption());
            },
            [3_u8] => {
                let state_msg = match power_plug.current_state(){
                    "Off" => [0u8],
                    "On" => [1u8],
                    _ => [2u8]
                };
                if handle_request(AnswerCode::Success, &stream, &state_msg,).is_err(){
                    continue
                }
                println!("Current state of power plug {} is {}", power_plug.get_name(), power_plug.current_state());
            },
            [4_u8] => {
                let plug_name = power_plug.get_name();
                if handle_request(AnswerCode::Success, &stream, plug_name.as_bytes(),).is_err(){
                    continue
                }
                println!("Name of power plug is {}", power_plug.get_name())
            }
            _ => {handle_request(AnswerCode::InternalError, &stream, &[]).unwrap();}
        }
        println!("Command from {:?} completed", sender)
    }
}

fn handle_connection(mut tcp_stream: &TcpStream) -> [u8; 1] {
    let mut data = [0_u8; 1];
    match tcp_stream.read_exact(&mut data) {
        Ok(()) => {},
        Err(e) => {
            println!("Error {} occurred", e);
            tcp_stream.shutdown(Shutdown::Both).unwrap();
        },
    }
    data
}
