
use std::cmp;

fn merge_overlapping_intervals(arr: &mut Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    arr.sort_by(|a,b| a[0].cmp(&b[0]));
    
    let mut result: Vec<Vec<i32>> = Vec::new();
    result.push(arr[0].clone());

    for i in 1..arr.len() {
        let current: Vec<i32> = arr[i].clone();
        let j: usize = result.len() - 1;

        if current[0] >= result[j][0] && current[0] <= result[j][1] {
            result[j][1] = cmp::max(current[1],result[j][1]);
        } else {
            result.push(current);
        }
    }
    result
}

fn main() {
    let mut arr: Vec<Vec<i32>> = vec![
        vec![89, 90], vec![-10, 20], vec![-50, 0],
        vec![70, 90], vec![90, 91], vec![90, 95]
    ];
    let result: Vec<Vec<i32>> = merge_overlapping_intervals(&mut arr);

    println!("Result: {:?}", result);
}



    // Array = [
    //     [89, 90], [-10, 20], [-50, 0],
    //     [70, 90], [90, 91],  [90, 95]
    // ]

    // Result = [
    //     [-50, 20], [70, 95]
    // ]


