fn lazy_adder(a: u32, b: u32) -> impl Fn() -> u32 {
    move || (a + 1) as u32
}

fn main() {
    let add_val = lazy_adder(10, 10);
    println!("{:?}", add_val());
}