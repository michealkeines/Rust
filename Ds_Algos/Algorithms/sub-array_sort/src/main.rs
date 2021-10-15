use std::cmp;

fn sub_array_sort(arr: &mut [i32]) -> Vec<i32> {
    let mut out_of_order_max: i32 = -i32::MAX;
    let mut out_of_order_min: i32 = i32::MAX;

    for i in 0..arr.len() {
        let current: i32 = arr[i];
        if is_out_of_order(i, current, arr) {
            out_of_order_max = cmp::max(current, out_of_order_max);
            out_of_order_min = cmp::min(current, out_of_order_min);
        }
    }

    if out_of_order_max == -i32::MAX && out_of_order_min == i32::MAX { return vec![-1,-1]; }

    let mut index_of_min_out_of_order: usize = 0;
    while arr[index_of_min_out_of_order] <= out_of_order_min { index_of_min_out_of_order += 1 }

    let mut index_of_max_out_of_order: usize = arr.len() - 1;
    while arr[index_of_max_out_of_order] >= out_of_order_max { index_of_max_out_of_order -= 1 }
    
    vec![index_of_min_out_of_order as i32, index_of_max_out_of_order as i32]
}

fn is_out_of_order(i: usize, current: i32, arr: &mut [i32]) -> bool {
    if i == 0 { return arr[i+1] < current; }
    if i == arr.len() - 1 { return arr[i-1] > current; }
    return arr[i-1] > current || arr[i+1] < current;
}

fn main() {
    let mut arr: Vec<i32> = vec![1, 2, 4, 7, 10, 11, 7, 12, 7, 7, 16, 18, 19];

    let result: Vec<i32> = sub_array_sort(&mut arr);
    println!("Result: {:?}",result);
}
