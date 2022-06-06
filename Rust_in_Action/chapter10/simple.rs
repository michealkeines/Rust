fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let a = 2;
    let b = 3;
    let add_c = || {a + b};

    assert_eq!(add(a,b), add_c());
}