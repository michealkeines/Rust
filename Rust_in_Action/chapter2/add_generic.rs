use std::ops::Add;
use std::time::{Duration};

fn add<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn main() {
    let c = add(4,5);
    println!("{}", c);

    let durations = add(Duration::new(5,0), Duration::new(10,0));

    println!("{:?}", durations);
}