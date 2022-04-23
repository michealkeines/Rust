fn main() {
    let mut b = 4;
    {
        let mut a = 43 + b;
        a += 1;
    }

    b = a;
}