fn take_the_n(n: &mut u8) {
    *n += 2;
}

fn take_the_s(s: &mut String) {
    s.push_str("ing");
}

fn main() {
    let mut n = 5;
    let mut s = String::from("test");

    take_the_n(&mut n);
    take_the_s(&mut s);

    println!("n changes to {}", n);
    println!("s changes to {}", s);
}