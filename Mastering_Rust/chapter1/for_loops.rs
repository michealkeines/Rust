fn main() {
    print!("Normal ranges: ");
    for i in 0..10 {
        print!("{},", i);
    }

    println!();

    print!("Inclusive ranges: ");
    for i in 0..=10 {
        print!("{},", i);
    }
}
