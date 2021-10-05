fn three_number_sum(arr: &mut Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    arr.sort();
    let mut result: Vec<Vec<i32>> = vec![];
    for i in 0..arr.len() {
        let current = arr[i];
        let mut j: usize = i + 1;
        let mut k: usize = arr.len() - 1;

        while j < k {
            let s = current + arr[j] + arr[k];
            if s == target {
                result.push(vec![current, arr[j], arr[k]]);
                j += 1;
                k -= 1;
            } else if s < target {
                j += 1;
            } else {
                k -= 1;
            }
        }
    }  
    result
}

fn main() {
    let mut array: Vec<i32> = vec![12, 3, 1, 2, -6, 5, 0, -8, -1, 6];
    let target: i32 = 0;

    let result: Vec<Vec<i32>> = three_number_sum(&mut array, target);
    println!("Result : {:?}",result);
}


    //     array = [12, 3, 1, 2, -6, 5, 0, -8, -1, 6]

    //     Sorted Array = [-8, -6, -1, 0, 1, 2, 3, 5, 6, 12]

    //     We Know that every element in this array is in increasing order, as we iterate through the array we could have pointers to two elements, that is smallest and largest numbers.

    //     we will start with the first element 

    //     target_sum = 0

    //     index of current = 0
    //     index of smallest = current + 1 //as we already have -8 in current, the next value must be the smallest value

    //     index of largest = final element in array //the final value must be the largest value

    //     our equatation is 
    //     => current_sum = current + smallest + largest

    //  If our current_sum is equal to target_sum, then we got the triplet, we could store it in a seperate array.

    //  if our current_sum is smaller than target_sum, that means we need to add bigger value, so we increment the index of smaller value, thus we will get a value that is greater than the previous value.

    //  if our current_sum is greater than target_sum, that means we need to add smaller value, so we decrement the index of largest value, thus we will get a value that is smaller than the previous value.

















     

     result = [
        [-8, 2, 6], 
        [-8, 3, 5], 
        [-6, 0, 6], 
        [-6, 1, 5], 
        [-1, 0, 1]
    ]




