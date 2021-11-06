
fn same_bst(arr1: &Vec<i32>, arr2: &Vec<i32>) -> bool {
    is_valid(arr1,arr2)
}

fn is_valid(arr1: &Vec<i32>, arr2: &Vec<i32>) -> bool {
    if arr1.len() != arr2.len() { return false; }

    if arr1.len() == 0 && arr2.len() == 0 { return true; }

    if arr1[0] != arr2[0] { return false; }

    let leftarr1: Vec<i32> = get_left(arr1);
    let leftarr2: Vec<i32> = get_left(arr2);
    let rightarr1: Vec<i32> = get_right(arr1);
    let rightarr2: Vec<i32> = get_right(arr2);

    return is_valid(&leftarr1,&leftarr2) && is_valid(&rightarr1, &rightarr2);
}


fn get_left(arr: &Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for i in 1..arr.len() {
        if arr[i] < arr[0] { result.push(arr[i]); }
    }
    return result;
}

fn get_right(arr: &Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for i in 1..arr.len() {
        if arr[i] >= arr[0] { result.push(arr[i]); }
    }
    return result;
}


fn main() {
    let arr1: Vec<i32> = vec![10, 15, 8, 12, 94, 81, 5, 2, 11];
    let arr2: Vec<i32> = vec![10, 8, 5, 15, 2, 12, 11, 94, 81];

    let result: bool = same_bst(&arr1, &arr2);
    println!("Result: {}",result);
}

