use serialization::message::Message;
use std::time::Instant;

fn main() {
        let ins = Instant::now();
        println!("{:?}", ins);
        let message = Message {
            value : String::from("This is my Message"),
            timestamp: ins
        };

        let serialized = serde_json::to_string(&message).unwrap();
        println!("{:?}", serialized);
        let deserialized: Message = serde_json::from_str(&serialized).unwrap();
        println!("{:?}", deserialized);
}
