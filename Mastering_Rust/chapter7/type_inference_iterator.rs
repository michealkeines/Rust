use std::fs::File;
use std::io::Read;

fn main() {
    let file = File::open("foo.txt").unwrap();
    let bytes: Vec<Result<u8,  _>> = file.bytes().collect();
}