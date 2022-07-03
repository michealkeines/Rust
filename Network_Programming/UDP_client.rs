use std::net::UdpSocket;
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8001").expect("could connet to socket");
    socket.connect("127.0.0.1:8000").unwrap();
    loop {
        let mut input = String::new();
        let mut buffer = [0;1500];

        std::io::stdin().read_line(&mut input).expect("input read error");

        socket.send(input.as_bytes()).unwrap();
        
        let (l, src) = socket.recv_from(&mut buffer).expect("read Error");
        println!("{}", std::str::from_utf8_mut(&mut buffer).unwrap());
    }
}