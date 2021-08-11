use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //let k = {let o = 1; o};
    let k = testing(23);
    println!("The value of k is: {}", k);
}

fn testing(x: i32) -> i32 {
    let value = 10;
    let arr: [u32; 5] = [1,2,3,4,5];
        let v = 10;
        let mut i = 0;
        let mut result: i32 = 0;
        while i < v {
            result = result + 1;
            i = i + 1;
        }
        let exp = result;
    println!("Result: {}",exp);
    for val in arr.iter() {
        println!("The value is: {}", val);
    }
    if (x < value) {
        return x;
    } else {
        return value;
    }
}
