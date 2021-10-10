
fn longest_peak(arr: &[i32]) -> u32 {
    let mut max_peak: u32 = 0;
    for i in 1..arr.len() - 1 {
        if arr[i] > arr[i-1] && arr[i] > arr[i+1] {
            let l: u32 = find_length(i, arr);
            if l > max_peak { max_peak = l; }
        }
    }
    max_peak
}

fn find_length(i: usize, arr: &[i32]) -> u32 {
    let mut count = 3;
    if i as i32 - 2 < 0 {
        let mut j = i + 2;
        while j < arr.len() - 1 && arr[i] > arr[i+1] {
            count += 1;
            j += 1;
        }
        
        return count;
    }

    let mut j = i - 2;
    while j >= 0 && arr[j] < arr[j+1] { 
        count += 1;
        if j == 0 { break; } 
        j -= 1;
    }
    j = i + 2;
    while j < arr.len() - 1 && arr[i] < arr[i-1] {
        count += 1;
        j += 1;
    }
    count
}


fn main() {
    let arr: Vec<i32> = vec![5, 4, 3, 2, 1, 2, 10, 12, -3, 5, 6, 4, 10];

    let result: u32 = longest_peak(&arr);
    println!("Result: {}",result);
}


