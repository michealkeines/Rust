
use std::rc::Rc;
use std::cell::RefCell;
use my_project::BST;


fn closest_value(tree: Rc<RefCell<BST>>, target: i32) -> i32 {
    let mut min_diff: i32 = (target - tree.borrow().value).abs();
    let mut _min_val: i32 = tree.borrow().value;

    let mut current = tree;

    loop {
        if !current.borrow().left.is_none() {
            let temp = (current.borrow().left.as_ref().unwrap().borrow().value - target).abs();
            if  temp < min_diff {
                min_diff = temp;
                _min_val = current.borrow().left.as_ref().unwrap().borrow().value;
            }
        }
        if !current.borrow().right.is_none() {
            let temp = (current.borrow().right.as_ref().unwrap().borrow().value - target).abs();
            if  temp < min_diff {
                min_diff = temp;
                _min_val = current.borrow().right.as_ref().unwrap().borrow().value;
            }
        }
        if target < current.borrow().value {
            if current.borrow().left.is_none() { break; }
            else {
                current = current.to_owned().borrow().left.to_owned().unwrap();
            }

        } else {
            if current.borrow().right.is_none() { break; }
            else {
                current = current.to_owned().borrow().right.to_owned().unwrap();
            }
        }
    }
    _min_val
}

fn main() {
    let bst = BST::new(10); // Root Node

    BST::insert(bst.clone(),  5);
    BST::insert(bst.clone(), 15);
    BST::insert(bst.clone(), 22);
    BST::insert(bst.clone(), 13);
    BST::insert(bst.clone(),  2);
    BST::insert(bst.clone(),  5);
    BST::insert(bst.clone(),  1);
    BST::insert(bst.clone(), 14);

    let target: i32 = 12;

    let result: i32 = closest_value(bst, target);
    println!("Result: {}",result);
}


    //Input Tree:
    //                 10
    //                /   \
    //               5     15
    //              / \   /  \
    //             2   5 13  22
    //            /        \
    //           1         14

    //Target = 12

    //Result = 13



