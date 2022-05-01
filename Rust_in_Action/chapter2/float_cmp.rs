fn main() {
    let a: f32 = 0.1;
    let b: f32 = 0.2;
    let c: f32 = 0.3;

    
    println!("0.1 + 0.2 = {:x}", (a+b).to_bits());
    println!("0.3: {:x}", c.to_bits());

    assert_eq!(a+b, c);
    assert_eq!(0.1 + 0.2, 0.3);
}