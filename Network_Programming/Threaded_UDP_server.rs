use std::io;
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8000")?;

    loop {
        let s = socket.try_clone().expect("clone failed");
        let mut buf = [0;10];
        match s.recv_from(&mut buf) {
            Ok((l, src)) => {
                std::thread::spawn( move || {
                    s.send_to(&buf, &src).expect("failed to respond");
                });
            },
            Err(_) => {eprintln!("failed to read from socket");}
        }
    }
    Ok(())
}