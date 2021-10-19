use std::collections::HashMap;
use std::cmp;

macro_rules! hashmap {
    ($($key: expr => $val: expr),*) => {{
        let mut map = ::std::collections::HashMap::new();
        $( map.insert($key, $val); )*
        map
    }}
}

fn minimum_distance(blocks: &Vec<HashMap<&str, bool>>, reqs: &Vec<&str>) -> u32 {
    let mut check: Vec<Vec<u32>> = Vec::new();
    for i in 0..reqs.len() {
        check.push(vec![]);
        check[i] = get_smallest_distance_vector(reqs[i],blocks);
    }
    let mut result: Vec<u32> = vec![0;blocks.len()];
    for k in 0..blocks.len() {
        let mut current: u32 = 0;
        for i in 0..reqs.len() {
            if check[i][k] > current {
                current = check[i][k];
            }
        }
        result[k] = current;
    }

    get_index_of_minimum_value(&result)
}

fn get_index_of_minimum_value(result: &Vec<u32>) -> u32 {
    let mut current: u32 = u32::MAX;
    let mut index: usize = 0;
    for i in 0..result.len() {
        if result[i] < current {
            current = result[i];
            index = i;
        }
    }
    index as u32
}

fn get_smallest_distance_vector(req: &str, blocks: &Vec<HashMap<&str, bool>>) -> Vec<u32> {
    let mut result: Vec<u32> = vec![u32::MAX;blocks.len()];
    for i in 0..blocks.len() {
        if blocks[i][req] == true { result[i] = 0; } 
        else {
            if i == 0 { continue; }
            else {
                if cmp::min(result[i-1], result[i]) != u32::MAX {
                    result[i] = cmp::min(result[i-1], result[i]) + 1
                }
            }
        }
    }
    for i in (0..blocks.len()-1).rev() {
        if blocks[i][req] == true { result[i] = 0; } 
        else {
            if cmp::min(result[i+1],result[i]) != result[i] && cmp::min(result[i+1], result[i]) != u32::MAX {
                result[i] = cmp::min(result[i+1],result[i]) + 1;
            }
        }
    }
    result
}


fn main() {
    let index0: HashMap<&str, bool> = hashmap!["gym" => false, "school" => true, "store" => false];
    let index1: HashMap<&str, bool> = hashmap!["gym" => true, "school" => false, "store" => false];
    let index2: HashMap<&str, bool> = hashmap!["gym" => true, "school" => true, "store" => false];
    let index3: HashMap<&str, bool> = hashmap!["gym" => false, "school" => true, "store" => false];
    let index4: HashMap<&str, bool> = hashmap!["gym" => false, "school" => true, "store" => true];

    let blocks: Vec<HashMap<&str, bool>> = vec![index0, index1, index2, index3, index4];

    let reqs: Vec<&str> = vec!["gym", "school", "store"];

    let result: u32 = minimum_distance(&blocks, &reqs);

    println!("Result : {}",result);

}

