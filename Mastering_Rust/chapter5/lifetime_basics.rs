struct SomeRef<T> {
    part: &T
}


fn main() {
    let a = SomeRef { part: &43 };
}