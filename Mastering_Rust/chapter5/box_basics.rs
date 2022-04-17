fn box_ref<T>(b: T) -> Box<T> {
    let a = b;
    Box::new(a)
}

#[derive(Debug)]
struct Foo {
    a: u32
}

fn main() {
    let boxed_one = Box::new(Foo{ a: 23 });
    let unboxed_one = *boxed_one;
    println!("values inside {}", unboxed_one.a);
    
  //  println!("box {:?}", box_ref(unboxed_one));
    println!("box value {}", box_ref(unboxed_one).a);
}