use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8000")?;

    loop {
        let mut s = String::new();

        let value = std::io::stdin().read_line(&mut s)?;
        
        stream.write(&s.as_bytes())?;
        let mut out = vec![0;5];
        stream.read_exact(&mut out)?;
        println!("{}",std::str::from_utf8_mut(&mut out).unwrap());
    }
    Ok(())
 
}