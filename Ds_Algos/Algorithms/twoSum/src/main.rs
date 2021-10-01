
    use std::collections::HashMap;

    fn two_sum( arr: &[i32], target_sum: i32 ) -> Vec<i32> {
        let mut hash = HashMap::new();

        for i in 0..arr.len() {
            let j = target_sum - arr[i];
            if hash.contains_key(&j) {
                return vec![arr[i],j];
            }
            hash.insert(arr[i],1);
        }
        vec![]
    }

    fn main() {
        let arr = vec![ -1, 4, 5, 7, 2];
        let target_sum: i32 = 7;
        
        let result: Vec<i32> = two_sum(&arr, target_sum);
        println!("Result : {:?}", result);
    }

// val = [ -1, 4, 5, 7, 2 ]

// target_sum = 7

// result = [ 5, 2 ]


//     List = [ possible Numbers ]
                                                                               
//     i = Current value                                                              
                                                                                
//     j = Unknown                                                                                                                                                  
//     target_sum = Given Value                                                                                                                                     
//     => i + j = target_sum // Equation                                                                                                                    
//     => j = target_sum - i // Unknown = target_sum - current


