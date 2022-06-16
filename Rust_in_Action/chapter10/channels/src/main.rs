#[macro_use]
extern crate crossbeam;

use std::thread;
use crossbeam::channel::unbounded;

use crate::ConnectivityCheck::*;

#[derive(Debug)]
enum ConnectivityCheck {
    Ping,
    Pong,
    Pang
}

fn main() {
    let number_of_messages = 3;
    let (requests_tx, requests_rx) = unbounded();
    let (responses_tx, responses_rx) = unbounded();

    thread::spawn(move || loop {
        match requests_rx.recv().unwrap() {
            Pong => eprintln!("unexpected pong response"),
            Ping => responses_tx.send(Pong).unwrap(),
            Pang => return,
        }
    });

    for _ in 0..number_of_messages {
        requests_tx.send(Ping).unwrap();
    }
    requests_tx.send(Pong).unwrap();
    requests_tx.send(Pang).unwrap();

    for _ in 0..number_of_messages {
        select! {
            recv(responses_rx) -> msg => println!("{:?}", msg),
        }
    }

    let (tx, rx) = unbounded();

    thread::spawn(move ||{
        tx.send(42).unwrap();
        
    });

    select!{
        recv(rx) -> msg => println!("{:?}", msg.unwrap())
    }
}
