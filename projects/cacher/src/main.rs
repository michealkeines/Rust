use std::process;

fn main() {
    let intensity: u32 = 3;
    let random_number: u32 = 5;
    if let Err(e) = cacher::run(intensity, random_number) {
        eprintln!("Error: {}",e);
        process::exit(1);
    }
}
