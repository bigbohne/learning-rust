use std::net::{TcpListener, TcpStream};
use std::io::BufRead; 
use std::io::BufReader;
use std::thread;

struct Command {
    command : String,
    key : String,
    value : String
}

impl Command {
    fn from_line(line: &String) -> std::io::Result<Self> {
        let split: Vec<&str> = line.split(' ').collect();

        if split.len() < 2 {
            return Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
        }

        let cmd = Command{command : String::new(), key : String::new(), value : String::new()};
        Ok(cmd)
    }
}

fn handle_client(stream: TcpStream) -> std::io::Result<()> {
    let reader = BufReader::new(stream);

    for line in reader.lines() {
        let command = Command::from_line(&line?)?;

        println!("{} {} {}", command.command, command.key, command.value)
    } 

    Ok(())   
}

fn handle_new_client(stream : std::io::Result<TcpStream>) {
    match stream {
            Ok(stream) => {
                match handle_client(stream) {
                    Err(error) => println!("oups: {}", error),
                    Ok(_) => ()
                }
                
            },
            Err(error) => println!("Client error: {}", error)
        }
}

fn main() {
    let listener = match TcpListener::bind("0.0.0.0:3000") {
        Ok(socket) => socket,
        Err(error) => {
            println!("{}", error);
            panic!(error);
        }
    };

    for stream in listener.incoming() {
        let _child = thread::spawn(move || {
            handle_new_client(stream);
        });
    }
}
