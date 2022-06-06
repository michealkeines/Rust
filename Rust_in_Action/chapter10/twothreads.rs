use std::{thread, time};

fn main() {
    let t1 = time::Instant::now();

    let handler1 = thread::spawn(||{
        let m = time::Duration::from_millis(300);
        thread::sleep(m.clone());
    });
    let handler2 = thread::spawn(||{
        let m = time::Duration::from_millis(300);
        thread::sleep(m.clone());
    });

    handler1.join().unwrap();
    handler2.join().unwrap();
    
    let t2 = time::Instant::now();

    println!("{:02?}", t2.duration_since(t1));
}