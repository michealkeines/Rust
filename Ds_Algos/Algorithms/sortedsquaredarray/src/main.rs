fn sorted_squares(arr: &[i32]) -> Vec<i32> {
    let mut result: Vec<i32> = vec![0; arr.len()];
    let mut start = 0;
    let mut end = arr.len() - 1;

    let mut i: usize = arr.len() - 1;
    while start <= end {
        if arr[start].abs() > arr[end].abs() {
            result[i] = arr[start] * arr[start];
            start += 1;
            if i == 0 { break; }
            i -= 1;
        } else {
            result[i] = arr[end] * arr[end];
            end -= 1;
            if i == 0 { break; }
            i -= 1;
        }
    }
    result
}

fn main() {
    let arr = vec![-7,-3,2,3,11];
    let result: Vec<i32> = sorted_squares(&arr);

    println!("Result: {:?}",result);
}
