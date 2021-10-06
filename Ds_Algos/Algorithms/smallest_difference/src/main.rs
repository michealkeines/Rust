
fn smallest_difference(arr1: &mut [i32], arr2: &mut [i32]) -> Vec<i32> {
    arr1.sort();
    arr2.sort();
    let mut i = 0;
    let mut j = 0;
    let mut current_min: Vec<i32> = vec![i32::MAX, 0, 0];

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] > arr2[j] {
            let temp_diff = arr1[i] - arr2[j];
            if temp_diff < current_min[0] {
                current_min = vec![temp_diff,i as i32,j as i32];
            }
            j += 1;
        } else {
            let temp_diff = arr2[j] - arr1[i];
            if temp_diff < current_min[0] {
                current_min = vec![temp_diff,i as i32,j as i32];
            }
            i += 1;
        }
    }
    [arr1[current_min[1] as usize],arr2[current_min[2] as usize]].to_vec()
}

fn main() {
    let mut arr1 = vec![240, 124, 86, 111, 2, 84, 954, 27, 89];
    let mut arr2 = vec![1, 3, 954, 19, 8];

    let result: Vec<i32> = smallest_difference(&mut arr1, &mut arr2);

    println!("Result: {:?}", result);
}


        Array One = [240, 124, 86, 111, 2, 84, 954, 27, 89]

        Array Two = [1, 3, 954, 19, 8]

        Smallest Difference = [ 954, 954 ] // Difference = 0


        // Sorted Arrays
        arr1 = [2, 27, 84, 86, 89, 111, 124, 240, 954]
        arr2 = [1, 3, 8, 19, 954]

        Smallest Difference also means that both values should be closest to each other or equal

        we can start with the pointers (i,j) to start of both array and compare both values,

        if the value in index i is equal to value in index j, that this would be smallest_difference we could possibly get, so we can return this.

        if the value in the index i is less than value in index j, then we have to look for better number, if we increment jth index, we will endup with a even bigger difference as value in jth index is already greater than value in i, however if we increment ith index, we may find a smaller or equal value to what is in j. similarly, if the value in the index j is less than value in index i, then we can increment j.

        we continue this untill both i,j less than their respective array lengths

