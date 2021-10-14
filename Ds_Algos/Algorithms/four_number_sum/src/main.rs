
use std::collections::HashMap;

fn four_number_sum(arr: &[i32], target_sum: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut check: HashMap<i32,Vec<Vec<i32>>> = HashMap::new();

    for i in 0..arr.len() {
        let current_val = arr[i];
        for j in i+1..arr.len() {
            let next_val = arr[j];
            let diff = target_sum - ( current_val + next_val );
            if check.contains_key(&diff) {
                let temp_arr: Vec<Vec<i32>> = check[&diff].clone();
                for k in 0..temp_arr.len() {
                    result.push(vec![current_val, next_val, temp_arr[k][0],temp_arr[k][1]]);
                }
            }
        }
        for k in 0..i {
            let prev_val = arr[k];
            let sum = current_val + prev_val;
            check.entry(sum)
                    .or_insert_with(Vec::new)
                    .push(vec![current_val,prev_val]);
        }
    }
    result
}

fn main() {
    let arr: Vec<i32> = vec![-2, -1, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let target_sum: i32 = 4;

    let result: Vec<Vec<i32>> = four_number_sum(&arr, target_sum);
    println!("Result: {:?}",result);
}


//     Array = [-2, -1, 1, 2, 3, 4, 5, 6, 7, 8, 9]

//     Target_Sum = 4

//     Result = [ 
//         [1, 6, -1, -2], // 1 + 6 + (-1) + (-2) = 4
//         [2, 3,  1, -2], // 2 + 3 +   1  + (-2) = 4
//         [2, 5, -1, -2], // 2 + 5 + (-1) + (-2) = 4
//         [3, 4, -1, -2]  // 4 + 4 + (-1) + (-2) = 4
//     ]





// Array = [7, 6, 4, -1, 1, 2]

// Target_Sum = 16


