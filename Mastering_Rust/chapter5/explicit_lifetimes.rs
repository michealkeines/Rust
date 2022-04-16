fn foo<'a, 'b>(a: &'a str, b: &'b str) -> &'b str {
    b
}

fn main() {
    let a = "hello";
    let b = "world";
    let c = foo(a, b);
}