use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let path = Path::new("data.txt");
    let file = File::open(&path);
    let mut s = String::new();
    if let Ok(f) = file.as_ref().unwrap() {
        file.as_ref().unwrap().read_to_string(&mut s);
    } else {

    }

    
    println!("{}",s);
}