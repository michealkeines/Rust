use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);

    let incremented_value = map.get("three").expect("test") + 1;
    println!("{}", incremented_value);
}

