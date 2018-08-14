use std::env::args;
use std::net::UdpSocket;
use std::str::from_utf8;

fn main() -> std::io::Result<()> {
    let arguments: Vec<String> = args().collect();
    let addr = &arguments[1];
    let socket = UdpSocket::bind(addr)?;

    let mut buf = [0; 100];
    let (amt, src) = socket.recv_from(&mut buf)?;
    println!("{}:{:?}:{}", amt, src, from_utf8(&buf).unwrap());
    Ok(())
}
