use std::collections::HashMap;

fn main() {
    let mut fruits = HashMap::new();
    fruits.insert("apple", 3);
    fruits.insert("mango", 3);
    fruits.insert("orange", 3);
    fruits.insert("avocado", 3);

    for (k, v) in &fruits {
        println!("I got {} {}", v, k);
    }

    fruits.remove("orange");
    let old_avacado = fruits["avocado"];
    fruits.insert("avocado", old_avacado + 5);
    println!("\n I now have {} avocados", fruits["avocado"]);
}
