fn test(s: String) {

}

fn main() {
    let mut a = String::from("Owned string");
    let a_ref = &mut a;
    a_ref.push('!');
    println!("{}", a);
}