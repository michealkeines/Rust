use std::cmp;

fn min_reward(arr: &[u32]) -> u32 {
    let mut rewards: Vec<u32> = vec![1;arr.len()];

    for i in 1..arr.len() {
        if arr[i - 1] < arr[i] {
            rewards[i] = rewards[i-1] + 1;
        }
    }
    for i in (0..arr.len()-1).rev() {
        if arr[i + 1] < arr[i] {
            rewards[i] = cmp::max(rewards[i+1] + 1,rewards[i]);
        }
    }
    println!("{:?}",rewards);
    rewards.iter().sum()
}

fn main() {
    let arr: Vec<u32> = vec![8, 4, 2, 1, 3, 6, 7, 9, 5];

    let result: u32 = min_reward(&arr);
    println!("Result: {}", result);
}
