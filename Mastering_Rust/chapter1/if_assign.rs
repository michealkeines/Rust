fn main() {
    let result: &str = if 1 == 2 {
        "Wait, what?"
    } else {
        "I can't believe you dragged me into this"
    };

    println!("You know what? {}", result);
}
