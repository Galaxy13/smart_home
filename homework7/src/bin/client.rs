use std::net::{TcpStream};
use std::io::{Read, stdin, Write};
use std::thread;
use std::time::Duration;

enum ClientCommand{
    ChangeStateOn,
    ChangeStateOff,
    GetConsumption,
    GetState,
    GetName,
    WrongCommand,
    ExitCode
}

impl ClientCommand {
    fn value(&self) -> &[u8]{
        match *self {
            ClientCommand::ChangeStateOn => &[0_u8],
            ClientCommand::ChangeStateOff => &[1_u8],
            ClientCommand::GetConsumption => &[2_u8],
            ClientCommand::GetState => &[3_u8],
            ClientCommand::GetName => &[4_u8],
            ClientCommand::WrongCommand => &[255_u8],
            ClientCommand::ExitCode => &[10_u8]
        }
    }
}

enum ExpectedAnswer{
    Power,
    State,
    Name,
    Other
}

fn handle_answer(stream: &mut TcpStream, expected_type: ExpectedAnswer) -> Result<Vec<u8>, std::io::Error>{
    let mut buf = Vec::with_capacity(1024);
    match stream.read_to_end(&mut buf) {
        Ok(_) => {
            let status_code = buf[0];
            match status_code {
                1u8 => println!("Request Successful"),
                100u8 => println!("Internal Server Error"),
                _ => println!("Unknown Error")
            }
            let msg_length = usize::from(buf[1]);
            if msg_length != 0 {
                let msg = &buf[2..2+msg_length];
                match expected_type {
                    ExpectedAnswer::Power => {
                        let msg = [msg[0], msg[1], msg[2], msg[3]];
                        let power = f32::from_be_bytes(msg).to_be_bytes().to_vec();
                        Ok(power)
                    }
                    _ => Ok(msg.into()),
                }
            } else { Ok(vec![]) }
        },
        Err(e) => Err(e),
    }
}

const HOST: &str = "127.0.0.1:23015";
fn main() {
    loop {
        let mut command_line = String::new();
        println!("Enter new command:");
        stdin().read_line(&mut command_line).unwrap();
        let command = handle_input(command_line.trim());
        match command {
                ClientCommand::ExitCode => break,
                ClientCommand::WrongCommand => println!("Wrong command!"),
                _ => {
                    let mut stream = TcpStream::connect(HOST).expect("Unable to connect to server");
                    stream.write_all(command.value()).expect("Server Unreachable");
                    thread::sleep(Duration::from_secs_f32(0.3));
                    match command {
                        ClientCommand::ChangeStateOn => {
                            handle_answer(&mut stream, ExpectedAnswer::Other).unwrap();
                            println!("Power state changed to On");
                        },
                        ClientCommand::ChangeStateOff => {
                            handle_answer(&mut stream, ExpectedAnswer::Other).unwrap();
                            println!("Power state changed to Off");
                        },
                        ClientCommand::GetState => {
                            let answer: Vec<u8> = handle_answer(&mut stream, ExpectedAnswer::State).unwrap();
                            match answer[0] {
                                0u8 => println!("Power State: OFF"),
                                1u8 => println!("Power State: ON"),
                                _ => println!("Unknown state")
                            }
                        },
                        ClientCommand::GetName => {
                            let answer: Vec<u8> = handle_answer(&mut stream, ExpectedAnswer::Name).unwrap();
                            let name = String::from_utf8(answer).unwrap();
                            println!("Power plug name: {}", name);
                        },
                        ClientCommand::GetConsumption => {
                            let answer: Vec<u8> = handle_answer(&mut stream, ExpectedAnswer::Power).unwrap();
                            let power = f32::from_be_bytes([answer[0], answer[1], answer[2], answer[3]]);
                            println!("Power cunsumption: {}", power);
                        },
                        ClientCommand::ExitCode => {
                            break
                        }
                        _ => {}
                    }
                }
        }
    }
}

fn handle_input(command: &str) -> ClientCommand {
    if !command.is_empty() {
        match command {
            "state on" => ClientCommand::ChangeStateOn,
            "state off" => ClientCommand::ChangeStateOff,
            "get power" => ClientCommand::GetConsumption,
            "get state" => ClientCommand::GetState,
            "get name" => ClientCommand::GetName,
            "exit" => ClientCommand::ExitCode,
            _ => ClientCommand::WrongCommand,
        }
    } else {
        ClientCommand::WrongCommand
    }
}