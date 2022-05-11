use std::fmt;
use std::fmt::Debug;

struct Message {
    to: u64,
    content: String
}

impl Debug for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Message")
         .field("to", &self.to)
         .field("content", &self.content)
         .finish()
    }
}

struct GroundStation;

impl GroundStation {
    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat {
            id: sat_id
        }
    }
    fn send(&self, mailbox: &mut Mailbox, msg: Message) {
        mailbox.post(msg);
    }
}

struct CubeSat {
    id: u64
}

impl CubeSat {
    fn recv(&self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

impl Debug for CubeSat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("CubeSat")
         .field("id", &self.id)
         .finish()
    }
}

enum StatusMessage {
    Ok
}

impl Debug for StatusMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StatusMessage::Ok => {
                write!(f, "OK")
            }
        }
    }
}

struct Mailbox {
    messages: Vec<Message>
}

impl Mailbox {
    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }
        None
    }
}

impl Debug for Mailbox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Mailbox")
         .field("messages", &self.messages)
         .finish()
    }
}

fn check_status(sat_id: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![1, 2, 3]
}

fn main() {
    let mut mail = Mailbox { messages: vec![] };

    let base = GroundStation {};

    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let sat = base.connect(sat_id);

        let msg = sat.recv(&mut mail);
        println!("{:?}: {:?}", sat, msg);
    }



    // let base = GroundStation {};
    // let mut sat_a = CubeSat {
    //     id: 0,
    //     mailbox: Mailbox {
    //         messages: vec![]
    //     }
    // };

    // println!("t0: {:?}", sat_a);

    // base.send(&mut sat_a, Message::from("Hello There!"));

    // println!("t1: {:?}", sat_a);

    // let msg = sat_a.recv();

    // println!("t2: {:?}", sat_a);
    // println!("msg: {:?}", msg);



//     let sat_a = CubeSat { id: 0, mailbox: Mailbox { messages: vec![] } };
//     let sat_b = CubeSat { id: 1, mailbox: Mailbox { messages: vec![] } };
//     let sat_c = CubeSat { id: 2, mailbox: Mailbox { messages: vec![] } };
//   //  println!("{:?}", sat_a);
//     let a_status = check_status(sat_a);
//     let b_status = check_status(sat_b);
//     let c_status = check_status(sat_c);
//     println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);

//     // let a_status = check_status(sat_a);
//     // let b_status = check_status(sat_b);
//     // let c_status = check_status(sat_c);
//     // println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);

}