use std::panic;

fn main() {
    panic::catch_unwind(
        || {
            panic!("Panicking");
        }
    ).ok();

    println!("Survived the paic!");
}