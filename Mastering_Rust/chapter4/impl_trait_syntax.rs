use std::fmt::Display;

fn show_me(val: impl Display) {
    println!("{}", val);
}

fn dont_me<T>(val: T) where T: Display {
    println!("{}", val);
}

fn main() {
    show_me(1);
    dont_me(2);
}