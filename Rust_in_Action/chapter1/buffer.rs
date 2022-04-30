fn main() {
    let fruit = vec!['q', 'q', 'w'];

    let overflow = fruit[4];
    assert_eq!(overflow, 'e');
}