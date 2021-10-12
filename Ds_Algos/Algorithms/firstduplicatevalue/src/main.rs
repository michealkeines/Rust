fn first_duplicate_value(arr: &mut [i32]) -> Option<u32> {
    for i in 0..arr.len() {
        let j = arr[i].abs() as usize - 1;
        if arr[j] < 0 { 
            return Some(arr[i].abs() as u32); 
        } 
        else { arr[j] = -(arr[j]); }
    }
    None
}

fn main() {
    let mut arr: Vec<i32> = vec![1,2,5,4,2,4];

    let result: Option<u32> = first_duplicate_value(&mut arr);
    
    match result {
        Some(val) => { println!("Result: {}",val); },
        None => { println!("No Duplicates."); }
    }
}
