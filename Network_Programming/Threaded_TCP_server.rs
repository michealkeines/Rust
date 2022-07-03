use std::net::{ TcpListener, TcpStream };
use std::io;
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    loop {
        let mut buff = vec![0;5];

        stream.read_exact(&mut buff).unwrap();

        stream.write(&mut buff);
    }
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")?;

    for stream in listener.incoming() {
        let mut s = stream.unwrap();
        std::thread::spawn(move || {
           handle_client(s);
        });
    }

    Ok(())
}