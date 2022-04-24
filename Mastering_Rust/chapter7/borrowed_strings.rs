fn get_str_literal<'a>() -> &'a str {
    "from function"
}

fn main() {
    let my_str = "This is borrowed";
    let from_function = get_str_literal();
    println!("{} {}", my_str, from_function);
}