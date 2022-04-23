fn main() -> Result<(), &'static str> {
    let s = vec!["apple", "mango", "banana"];
    let fourth = s.get(2).ok_or("I got only 3 Fruits");
    println!("{:?}", fourth);
    Ok(())
}