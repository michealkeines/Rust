
fn zig_zag_traverse(arr: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    let height = arr.len() - 1;
    let width = arr[0].len() - 1;

    let mut going_down: bool = true;

    let mut row: usize = 0;
    let mut col: usize = 0;

    while !out_of_bounds(row, col, height, width) {
        result.push(arr[row][col]);

        if going_down == true {
            if col == 0 || row == height {
                going_down = false;
                if row == height { col += 1; } 
                else { row += 1; }
            } else {
                row += 1;
                col -= 1;
            }
        } else {
            if row == 0 || col == width {
                going_down = true;
                if col == width { row += 1; } 
                else { col += 1; }
            } else {
                row -= 1;
                col += 1;
            }
        }
    }

    result
}

fn out_of_bounds(row: usize, col: usize, height: usize, width: usize) -> bool{
    row > height || col > width
}

fn main() {
    let arr: Vec<Vec<i32>> = vec![
    vec![1, 3, 4, 10, 11],
    vec![2, 5, 9, 12, 19],
    vec![6, 8, 13, 18, 20],
    vec![7, 14, 17, 21, 24],
    vec![15, 16, 22, 23, 25]
  ];

    let result: Vec<i32> = zig_zag_traverse(&arr);
    println!("Result: {:?}",result);
}

