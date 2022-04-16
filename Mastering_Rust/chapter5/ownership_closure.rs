#[derive(Debug, Clone)]
struct Foo;

fn main() {
    let a = Foo;

    let closure = || {
        let b = a.clone();
    };

    println!("{:?}", a);
}