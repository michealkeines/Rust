fn compute(i: i32) -> i32 {
    2 * i
}

fn main() {
    let result_msg = "done";

    let result = if result_msg == "done" {
        let some_work = compute(8);
        let stuff = compute(4);
        compute(2) + stuff
    } else {
        compute(1)
    };

    println!("{}", result);
}