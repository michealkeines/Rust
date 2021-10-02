fn is_subsequence(arr: &[i32], seq: &[i32]) -> bool {
    let mut i: usize = 0;
    let mut _current: i32 = seq[i];

    for j in 0..arr.len() {
        if i == seq.len() { return true; }
        if arr[j] == seq[i] {
            i += 1;
            if i < seq.len() { _current = seq[i]; }
        }
    }
    if i == seq.len() { return true; } 
    else { return false; }
}

fn main() {
  let arr = vec![3,7,9,4,13,0];
  let seq = vec![3,4,0];

  let result: bool = is_subsequence(&arr, &seq);

  println!("Result: {}", result);  
}
