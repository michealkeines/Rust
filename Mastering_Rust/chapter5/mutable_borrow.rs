fn main() {
    let mut a = String::from("Owned string");
    let a_ref = &mut a;
    let b_ref = &a;
    a_ref.push('1');
}