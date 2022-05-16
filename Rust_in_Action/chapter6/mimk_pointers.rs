static B: [u8; 4] = [1, 2, 3, 4];
static C: [u8; 5] = [5, 6, 7, 8, 0];

fn main() {
    let a = 42;
    let b = &B;
    let c = &C;

    println!("a: {}, b: {:p}, c: {:p}", a, b, c);
}