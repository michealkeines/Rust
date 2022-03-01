
use std::cmp;

fn max_subset_sum(arr: &Vec<u32>) -> u32 {
    match arr.len() {
        0 => 0,
        1 => arr[0],
        2 => cmp::max(arr[0], arr[1]),
        _ => {
            let mut max_sums: Vec<u32> = vec![0;arr.len()];
            max_sums[0] = arr[0];
            max_sums[1] = cmp::max(arr[0], arr[1]);
            for i in 2..arr.len() {
                max_sums[i] = cmp::max(
                    max_sums[i-1],
                    max_sums[i-2] + arr[i]
                );
            }
            max_sums[arr.len() - 1]
        }
    }
}

fn main() {
    let arr: Vec<u32> = vec![75, 105, 120, 75, 90, 135];

    let result: u32 = max_subset_sum(&arr);
    println!("Result: {}",result);
}
