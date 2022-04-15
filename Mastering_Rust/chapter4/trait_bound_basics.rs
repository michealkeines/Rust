
use std::ops::Add;
use std::fmt::Display;

fn add_thing<T>(fst: T, snd: T) where T: Add {
    let _ = fst + snd;
}

fn show_me<T>(val: T) -> u32 where T: Display {
    println!("{}", val);
    0
}

fn main() {
    add_thing(2, 2);
}