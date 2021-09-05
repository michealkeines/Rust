fn perimeter(n: u64) -> u64 {
    let mut prev = 0;
    let mut current = 1;
    let mut total = 1;
    let mut res = 0;
    for i in 0..n+1 {
        res = res + current;
        total = current + prev;
        prev = current;
        current = total;
    }
    4 as u64 * res
}

fn dotest(n: u64, exp: u64) -> () {
    assert_eq!(perimeter(n), exp)
}

#[test]
fn basics_perimeter() {
    dotest(5, 80);
    dotest(7, 216);
    dotest(20, 114624);
    dotest(30, 14098308);
}