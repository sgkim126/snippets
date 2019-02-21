use std::env;

use mio::net::{TcpListener, TcpStream};

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "server" => {
            let addr = args[2].parse().unwrap();
            println!("listen on {}", addr);
            let _listener = TcpListener::bind(&addr).unwrap();
            loop {
            }
        }
        "client" =>  {
            let addr = args[2].parse().unwrap();
            println!("connect to {}", addr);
            let _stream = TcpStream::connect(&addr).unwrap();
            loop {
            }
        }
        _ => {
            panic!("{} is not expected", args[1]);
        }
    }
}
