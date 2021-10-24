use std::collections::HashMap;

fn maximum_points(points: &Vec<Vec<i32>>) -> i32 {
    let mut max_points: i32 = 0;
    
    for i in 0..points.len() {
        let mut check: HashMap<String, i32> = HashMap::new();
        let current: Vec<i32> = points[i].clone();
        for j in i+1..points.len() {
            let temp = points[j].clone();
            let slope: String = find_slope_rtn_string(current[0], temp[0], current[1], temp[1]);
            let update = check.entry(slope).or_insert(1);
            *update += 1;
        }
        for (_slope,val) in &check {
            if val > &max_points {
                max_points = *val;
            }
        }
    }
    max_points
}

fn find_slope_rtn_string(x1: i32, x2: i32, y1: i32, y2: i32) -> String {
    let mut a: i32 = y2 - y1;
    let mut b: i32 = x2 - x1;
    if a != 0 && b != 0 {
        let d: i32 = gcd(a,b);
        a = a / d;
        b = b / d;
        if a < 0 {
            a *= -1;
            b *= -1;
        }
    }
    format!("{} : {}",a,b)
}

//https://www.hackertouch.com/greatest-common-divisor-in-rust.html
fn gcd(x: i32,y: i32) -> i32{
    let mut a: i32 = x;
    let mut b: i32 = y;
    if b > a {
        let val = a;
        a = b;
        b = val;
    }
    loop {
        let res = a % b;
        if res == 0 { return b; }
        a = b;
        b = res;
    }
}

fn main() {
    let points: Vec<Vec<i32>> = vec![
        vec![1, 1],
        vec![2, 2],
        vec![3, 3],
        vec![0, 4],
        vec![-2, 6],
        vec![4, 0],
        vec![2, 1]
    ];
    let result: i32 = maximum_points(&points);
    println!("Result: {}",result);
}

