struct Number<'a> {
    num: &'a u8 
}

fn main() {
    let n = Number { num: &45 };
}