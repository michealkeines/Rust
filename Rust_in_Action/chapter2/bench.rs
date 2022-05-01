use std::time::{Duration, Instant};

fn main() {
    let mut count = 0;
    let time_limit = Duration::new(1,0);
    let start = Instant::now();

    while (Instant::now() - start) < time_limit {
        count += 1;
    }

    println!("{}", count);
}