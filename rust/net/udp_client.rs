use std::env::args;
use std::net::{SocketAddr, UdpSocket};
use std::str::FromStr;

fn main() {
    let arguments: Vec<String> = args().collect();
    let server = &arguments[1];
    let client = &arguments[2];

    let socket = UdpSocket::bind(server).unwrap();
    let to = SocketAddr::from_str(client).unwrap();

    let buf = "Some string";
    socket.send_to(buf.as_ref(), &to).unwrap();
}
