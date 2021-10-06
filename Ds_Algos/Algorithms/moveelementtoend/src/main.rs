
fn move_element_to_end(arr: &mut [i32], target: i32) {
    let mut i: usize = 0;
    let mut j: usize = arr.len() - 1;

    while i < j {
        if arr[i] == target && arr[j] == target {
            j -= 1;
        } else if arr[i] == target && arr[j] != target {
            swap(i, j, arr);
            i += 1;
            j -= 1;
        } else {
            i += 1;
        }
    }
}

fn swap(i: usize, j: usize, arr: &mut [i32]) {
    let temp: i32 = arr[j];
    arr[j] = arr[i];
    arr[i] = temp;
}

fn main() {
    let mut arr: Vec<i32> = vec![2, 4, 2, 5, 6, 2, 2];
    let target: i32 = 2;

    move_element_to_end(&mut arr, target);
    println!("Result: {:?}", arr);
}

