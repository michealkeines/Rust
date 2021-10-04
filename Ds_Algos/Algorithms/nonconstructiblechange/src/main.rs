
fn non_constructible_change(coins: &mut [u32]) -> u32 {
    coins.sort();
    let mut previous_change: u32 = 0;
    for i in 0..coins.len() {
        let current_coin = coins[i];
        if previous_change + 1 >= current_coin {
            previous_change += current_coin;
        } else {
            return previous_change + 1;
        }
    }
    previous_change + 1
}

fn main() {
    let mut coins: Vec<u32> = vec![5, 6, 1, 1, 2, 3, 4, 9];
    let result: u32 = non_constructible_change(&mut coins);

    println!("Result: {}", result);
}




    // Coins = [5, 6, 1, 1, 2, 3, 4, 9]

    // result = 32




    //     coins = [1,1,3]

    //     These are the possible combinations

    //     // 1, 1 + 1, 3, 3 + 1, 3 + 1 + 1

    //     // (previous_change, current_coin) = (0,1) = 1
    //     // (previous_change, current_coin) = (1,1) = 2
    //     // (previous_change, current_coin) = (2,3) = 5

    //     coins = [1,1,4]

    //     These are the possible combinations

    //     // 1, 1 + 1, 4, 4 + 1, 4 + 1 + 1
    //     // 3 is not possible

    //     // (previous_change, current_coin) = (0,1) = 1
    //     // (previous_change, current_coin) = (1,1) = 2
    //     // ( 4 > previous_change + 1),
    //     //      Thus 3 (previous_change+1) is not possible
    //     // (previous_change, current_coin) = (2,4) = 6  
        
    



