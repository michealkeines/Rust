use std::fmt::Display;


struct Foo<T> where T: Display {
    bar: T
}


struct test {
    b: String
}

fn main() {
    let r = Foo {
        bar: test { b: "ets".to_string() }
    };

    println!("{}",r.bar);
}