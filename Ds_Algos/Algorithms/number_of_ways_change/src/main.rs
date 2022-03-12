
fn number_of_ways(amount: u32, denoms: &Vec<u32>) -> u32 {
    let size: usize = amount as usize + 1; // Convert input type integer 32 to usize and add 1 to it
    let mut ways: Vec<u32> = vec![0;size]; // Create an empty array of length size with zeros [0,0,0,0,0,0...]
    ways[0] = 1; // Initialize index 0 with value 1

    // Iterate through every denom(coin) from input array and for every coin interate through another for loop  
    for denom in denoms.iter() { 
        for current_amount in 0..ways.len() {
            let temp = *denom as usize; // converting denom which is of integer 32 to usize
            if temp <= current_amount {
                ways[current_amount] = ways[current_amount] + ways[current_amount - temp]
            }
        }
    }
    ways[amount as usize] // returning final index in the array
}

/* 
  Given an array of distinct positive integers representing coin denominations and a
  single non-negative integer  representing a target amount of
  money, write a function that returns the number of ways to make change for
  that target amount using the given coin denominations.
*/

fn main() {
    let amount: u32 = 10;  // Initialize input amount as 10
    let denoms: Vec<u32> = vec![1,5,10,25]; // Initialize input array of coins [1, 5, 10, 25]

    let result: u32 = number_of_ways(amount, &denoms); // Call the function to find the number of possible changes by passing amount and coins as parameters
    println!("Result: {}", result); // Print the result
}
