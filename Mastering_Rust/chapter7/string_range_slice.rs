fn main() {
    let my_str = String::from("string are cool");

    let first_three = &my_str[0..3];
    println!("{:?}", first_three);
}