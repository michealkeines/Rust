use std::{thread, time};

fn main() {
    let t1 = time::Instant::now();

    for _ in 1..10 {
        let mut handlers = Vec::with_capacity(50);
        for _i in 1..4 {
            let pause = time::Duration::from_millis(20);
            let handle = thread::spawn(
                move || {
                    let start = time::Instant::now();
        
                    while start.elapsed() < pause {
                        thread::yield_now();
                    }
                }
            );
            handlers.push(handle);
        }

        while let Some(handle) = handlers.pop() {
            handle.join().unwrap();
        }
    }

    let t2 = time::Instant::now();

    println!("{:02?}",t2.duration_since(t1));
}