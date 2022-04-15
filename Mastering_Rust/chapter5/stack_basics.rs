fn double_of<'a>(b: &'a i32) -> &'a i32 {
    &b
}

fn main() {
    let a = 12;
    let result = double_of(&a);
    println!("{}",result);
}