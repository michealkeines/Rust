#[derive(Debug)]
struct Foo(u32);

fn main() {
    let foo = 2048;
    let bar = foo;

    println!("Foo is {:?}", foo);
    println!("Bar is {:?}", bar);
}