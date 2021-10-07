
fn is_monotonic(arr: &[i32]) -> bool {
    if arr.len() <= 2 { return true; }

    let mut direction: i32 = arr[1] - arr[0];
    for i in 2..arr.len() {
        if direction == 0 {
            direction = arr[i] - arr[i-1];
            continue;
        }

        if breaks_direction(direction, arr[i], arr[i-1]) { return false; }
    }
    true
}

fn breaks_direction(direction: i32, current: i32, previous: i32) -> bool {
    let difference = current - previous;
    if direction > 0 { return difference < 0; } 
    difference > 0
}

fn main() {
    let arr: Vec<i32> = vec![-1,-1,-7,-98,-99,-99,-188,-199];

    let result: bool = is_monotonic(&arr);

    match result {
        true => { println!("Array is Monotonic"); },
        false => { println!("Array is Non-Monotonic") }
    };
}

// An Array is said to be Monotonic if its elements from right to left are entirely non-increasing or non-decreasing

// [ 1, 2, 3, 3, 4, 5 ] // Non-Decreasing - Monotonic: every elmenet is increasing except 3, 3

// [ -1, -2, -3, -3. -4, -5 ] // Non-Increasing - Monotonic: every element is decreasing except -3, -3

// [ 1, 3, 7, 9, 1 ] // Non-Monotonic, as its not completely Non-decreasing as 9 is less then 1

// [ -1, -3, -7, -9, -1 ] // Non-Monotonic, as its not completely Non-Increasing as -1 is greater then -9


//                 Array = [-1,-1,-7,-98,-99,-99,-188,-199]

//                 Result = True



