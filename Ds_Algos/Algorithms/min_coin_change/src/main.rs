use std::cmp;

fn min_number_of_change(n: i32, denoms: Vec<u32>) -> i32 {
    let mut ways: Vec<i32> = vec![i32::MAX;n as usize + 1];
    ways[0] = 0;

    for denom in denoms.iter() {
        for current in 0..ways.len() {
            if *denom <= current as u32 {
                ways[current as usize] = cmp::min(ways[current as usize], 1 + ways[current as usize - *denom as usize])
            }
        }
    }
    if ways[n as usize] != i32::MAX {
        ways[n as usize]
    } else {
        -1
    }
}


fn main() {
    let denoms: Vec<u32> = vec![1, 5, 10];
    let n: i32 = 4;

    let result: i32 = min_number_of_change(n, denoms);
    println!("Result: {}", result);
}
