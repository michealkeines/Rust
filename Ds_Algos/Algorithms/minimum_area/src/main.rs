
use std::cmp;
use std::collections::HashSet;

fn minimum_area(points: &Vec<Vec<i32>>) -> i32 {
    let mut check: HashSet<String> = HashSet::new();
    let mut min_area: i32 = i32::MAX;

    for i in 0..points.len() {
        check.insert(to_string(points[i][0],points[i][1]));
    }

    for i in 0..points.len() {
        let current: Vec<i32> = points[i].clone();
        for j in i..points.len() {
            if i != j {
                let temp: Vec<i32> = points[j].clone();
                if current[0] != temp[0] && current[0] != temp[1] && current[1] != temp[0] && current[1] != temp[1] {
                    if check.contains(&to_string(current[0],temp[1])) && check.contains(&to_string(temp[0],current[1])) {
                        let l: i32 = cmp::max(current[0],temp[0]) - cmp::min(current[0],temp[0]);
                        let b: i32 = cmp::max(current[1],temp[1]) - cmp::min(current[1],temp[1]);
                        let val: i32 = l * b;
                        if val < min_area { min_area = val; }
                    }

                }
            }
        }
    }
    if min_area == i32::MAX { 0 }
    else { min_area }
}

fn to_string(x: i32, y: i32) -> String {
    format!("{} , {}",x,y)
}


fn main() {
    let points: Vec<Vec<i32>> = vec![
    vec![1, 5],
    vec![5, 1],
    vec![4, 2],
    vec![2, 4],
    vec![2, 2],
    vec![1, 2],
    vec![4, 5],
    vec![2, 5],
    vec![-1, -2]
  ];
  let result: i32 = minimum_area(&points);
  println!("Result: {}",result);
}

