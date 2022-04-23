fn main() {
    let precompute = {
        let a = (-34i64).abs();
        let b = 345i64.pow(3);
        let c = 3;
        a + b + c
    };

    let result_msg = match precompute {
        42 => "done",
        a if a % 2 == 2 => "continue",
        _ => panic!("Oh no!")
    };

    println!("{}", result_msg);
}