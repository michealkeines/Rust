use std::collections::HashMap;

fn largest_range(arr: &[i32]) -> Vec<i32> {
    let mut check: HashMap<i32, bool> = HashMap::new();

    for i in 0..arr.len() { check.insert(arr[i], true); }

    let mut start_val = 0;
    let mut end_val = 0;

    let mut max_count = -i32::MAX;
    for i in 0..arr.len() {
        let mut current_start: i32 = arr[i];
        let mut current_end: i32 = arr[i];

        let mut left_val: i32 = arr[i] - 1;
        while check.contains_key(&left_val) && check.get(&left_val) == Some(&true) {
            let t = check.entry(left_val).or_insert(false);
            *t = false;
            current_start = left_val;
            left_val -= 1;
        }

        let mut right_val: i32 = arr[i] + 1;
        while check.contains_key(&right_val) && check.get(&right_val) == Some(&true) {
            let t = check.entry(right_val).or_insert(false);
            *t = false;
            current_end = right_val;
            right_val += 1;
        }

        if (current_end - current_start) > max_count {
            start_val = current_start;
            end_val = current_end;
            max_count = current_end - current_start;
        }
    }
    vec![start_val, end_val]
}

fn main() {
    let arr: Vec<i32> = vec![8, 4, 2, 10, 3, 6, 7, 9, 1];

    let result: Vec<i32> = largest_range(&arr);
    println!("Result: {:?}",result);
}


