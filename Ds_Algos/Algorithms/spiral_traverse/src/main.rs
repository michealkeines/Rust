
fn spiral_traverse(arr: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    let mut row_start: usize = 0;
    let mut row_end: usize = arr.len() - 1;
    let mut col_start: usize = 0;
    let mut col_end: usize = arr[0].len() - 1;

    while row_start <= row_end && col_start <= col_end {
        for col in col_start..col_end+1 {
            result.push(arr[row_start][col]);
        }
        for row in row_start+1..row_end+1 {
            result.push(arr[row][col_end]);
        }
        for col in (col_start..col_end).rev() {
            if row_start == row_end { break; }
            result.push(arr[row_end][col]);
        }
        for row in (row_start+1..row_end).rev() {
            if col_start == col_end { break; }
            result.push(arr[row][col_start])
        }
        row_start += 1; 
        row_end -= 1; 
        col_start += 1; 
        col_end -= 1;
    }
    result
}

fn main() {
    let arr: Vec<Vec<i32>> = vec![ vec![1, 2, 3, 4], vec![12, 13, 14, 5], 
        vec![11, 16, 15, 6], vec![10, 9, 8, 7] ];
    let result: Vec<i32> = spiral_traverse(&arr);
    println!("Result: {:?}",result);
}


