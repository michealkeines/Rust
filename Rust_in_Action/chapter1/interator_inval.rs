fn main() {
    let mut letters = vec![
        "a", "b", "c"
    ];

    for letter in &mut letters {
        println!("{}", letter);
        letters.push(letter.clone());
    }
}