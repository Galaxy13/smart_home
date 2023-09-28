use std::net::{TcpStream};
use std::io::{stdin, Write};

const HOST: &str = "127.0.0.1:23015";
fn main() {
    loop {
        let mut stream = TcpStream::connect(HOST).expect("Unable to connect to server");
        let mut command_line = String::new();
        println!("Enter new command: \n");
        stdin().read_line(&mut command_line).unwrap();
        let command: Vec<&str> = command_line.split_whitespace().collect();
        if !command.is_empty() {
            let main_action = command[0];
            match main_action {
                "exit" => break,
                _ => {
                    if command.get(1).is_some(){
                        let argument = command[1];
                        stream.write_all(format!("{} {}", main_action, argument).as_bytes()).expect("Request failed")
                    }
                },
            }
        } else {
            println!("Enter the command")
        }
    }
}