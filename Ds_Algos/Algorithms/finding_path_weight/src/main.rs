
fn waterfall_streams(arr: &Vec<Vec<f32>>, source: usize) -> Vec<f32> {
    let mut above: Vec<f32> = arr[0].clone();
    above[source] = -1.0;

    for i in 1..arr.len() {
        let mut current: Vec<f32> = arr[i].clone();
        for j in 0..current.len() {
            if above[j] != 0.0 && above[j] != 1.0 {
                if current[j] != 1.0 {
                    current[j] += above[j];
                } else {
                    let mut left: usize  = j - 1;
                    let mut right: usize = j + 1;

                    while left >= 0 {
                        if left > 0 && current[left] == 1.0 && above[left] != 1.0 {
                            left -= 1;
                        } else if current[left] != 1.0 && above[left] != 1.0 {
                            current[left] += above[j] / 2.0;
                            break;
                        } else { break; }
                    }

                    while right <= current.len() - 1 {
                        if right < current.len() - 1 && current[right] == 1.0 && above[right] != 1.0 {
                            right += 1;
                        } else if current[right] != 1.0 && above[right] != 1.0 {
                            current[right] += above[j] / 2.0;
                            break;
                        } else { break; }
                    }

                }
            }
        }
        above = current;
    }

    for i in 0..above.len() {
        if above[i] != 0.0 { 
            above[i] = above[i] * -100.0;
        } 
    }
    above
}



fn main() {
    let arr: Vec<Vec<f32>> =  vec![
    vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    vec![0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0],
    vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    vec![1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0],
    vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0],
    vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
  ];
    
  let source: usize = 3;

  let result: Vec<f32> = waterfall_streams(&arr, source);
  println!("Result: {:?}",result);
}


// let arr: Vec<Vec<f32>> = vec![
//         vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], 
//         vec![0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0], 
//         vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], 
//         vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0], 
//         vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], 
//         vec![0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0], 
//         vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], 
//         vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0], 
//         vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], 
//         vec![0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0], 
//         vec![0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0], 
//         vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
//     ];
// let source: usize = 8;

