fn give_me<T: std::fmt::Display>(value: T) {
    let _ = value;
    println!("{}",value);
}

fn main() {
    let a = "generics";
    let b = 1024;
    give_me(a);
    give_me(b);
   // give_me((1,2));
}