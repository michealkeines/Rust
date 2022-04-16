#[derive(Clone, Debug)]
struct Dummy {
    items: Vec<u32>
}

fn main() {
    let mut a = Dummy { items: vec![54] };
    let mut b = a.clone();
    a.items.push(2);
    b.items.push(122);
    println!("a: {:?} , b:{:?}", a, b);
}