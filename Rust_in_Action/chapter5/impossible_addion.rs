#[allow(arithmetic_overflow)]

fn main() {
    let (a, b): (u16, u16) = (40_000, 30_000);

    let c: u16 = a + b;
    println!("{}", c);
}