use my_project::BST;

fn right_smaller_than(arr: Vec<i32>) -> Vec<i32> {
    if arr.len() == 0 {
        return vec![];
    }
    let mut result = arr.clone();
    let root = BST::new(arr[arr.len()-1],0,arr.len()-1);

    for i in (0..arr.len()-1).rev() {
        root.borrow_mut().insert(arr[i],0,i);
    }
    root.borrow_mut().update(&mut result);
    return result;
}


fn main() {
    let arr: Vec<i32> = vec![8, 5, 11, -1, 3, 4, 2];

    let result: Vec<i32> = right_smaller_than(arr);
    println!("Result: {:?}",result);
}

