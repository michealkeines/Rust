fn main() {
    let hello = String::from("Hello");
    for c in hello.chars() {
        print!("{}",c);
    }
    println!();
}