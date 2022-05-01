fn main() {
    let x = (-42.0_f32).sqrt();
    println!("{:?}", x);
    let y: f32 = 1.0/0.0;
    assert!(x.is_infinite());
}