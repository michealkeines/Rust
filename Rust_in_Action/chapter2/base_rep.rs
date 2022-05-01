fn main() {
    let three = 0b11;
    let thirty = 0o36;
    let three_hundred = 0x12c;

    println!("baase 10: {} {} {}", three, thirty, three_hundred);
    println!("baase 2: {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("baase 8: {:o} {:o} {:o}", three, thirty, three_hundred);
    println!("baase 16: {:x} {:x} {:x}", three, thirty, three_hundred);
}