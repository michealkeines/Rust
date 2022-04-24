const HEADER: &'static [u8; 4] = b"OBJ\0";

fn main() {
    println!("{:?}", HEADER);
}