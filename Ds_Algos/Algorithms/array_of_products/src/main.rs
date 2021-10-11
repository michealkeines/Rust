
fn array_of_products(arr: &[i32]) -> Vec<i32> {
    let mut left_values: Vec<i32> = vec![1; arr.len()];
    let mut right_values: Vec<i32> = vec![1; arr.len()];

    let mut result: Vec<i32> = Vec::new();

    let mut current_product = 1;
    for i in 1..arr.len()+1 {
        left_values[i-1] = current_product;
        current_product = current_product * arr[i-1];
    }

    let mut current_product = 1;
    for i in (0..arr.len()-1).rev() {
        right_values[i+1] = current_product;
        current_product = current_product * arr[i+1];
        if i == 0 { right_values[i] = current_product; }
    }

    for i in 0..arr.len() {
        result.push(left_values[i] * right_values[i]);
    }
    result
}

fn main() {
    let arr: Vec<i32> = vec![1, 8, 6, 2, 4];

    let result: Vec<i32> = array_of_products(&arr);
    println!("Result: {:?}",result);
}



Left_values = [1, 1, 8, 48, 96]

Right_values = [384, 48, 8, 4, 1]





// Array = [1, 8, 6, 2, 4]

// // Array[0] = 8 * 6 * 2 * 4 = 384
// // Array[1] = 1 * 6 * 2 * 4 = 48
// // Array[2] = 1 * 8 * 2 * 4 = 64
// // Array[3] = 1 * 8 * 6 * 4 = 192
// // Array[4] = 1 * 8 * 6 * 2 = 96

// Result =  [384, 48, 64, 192, 96]


//                                     [1, 8, 6, 2, 4]


// product = values to the left of index * values to the right of index





