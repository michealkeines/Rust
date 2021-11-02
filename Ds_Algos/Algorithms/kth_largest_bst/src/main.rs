use my_project::BST;
use std::rc::Rc;
use std::cell::RefCell;

type Node = Rc<RefCell<BST>>;

pub struct Check {
    value: i32,
    visited: usize
}

fn kth_largest(tree: Node, k: usize) -> i32 {
    let mut check = Check { value: 0, visited: 0 };
    helper(Some(tree), &mut check, k);
    check.value
}

fn helper(tree: Option<Node>, check: &mut Check, k: usize) -> Option<bool> {
    if tree.is_none() || check.visited >= k { return None; }

    let current_tree = tree.unwrap();
    helper(current_tree.to_owned().borrow().right.to_owned(),check,k);

    if k > check.visited {
        check.visited += 1;
        check.value = current_tree.borrow().value;
        helper(current_tree.to_owned().borrow().left.to_owned(),check,k);
        return None;
    } else {
        return None;
    }
}



fn main() {
    let bst = BST::new(15); // Root Node

    BST::insert(bst.clone(),  5);
    BST::insert(bst.clone(), 20);
    BST::insert(bst.clone(),  2);
    BST::insert(bst.clone(),  5);
    BST::insert(bst.clone(), 17);
    BST::insert(bst.clone(), 22);
    BST::insert(bst.clone(),  1);
    BST::insert(bst.clone(),  3);



    //          15
    //        /    \
    //       5      20
    //     /   \   /  \
    //    2     5 17  22
    //   / \          
    //  1   3
    
    let k: usize = 3;
    let result: i32 = kth_largest(bst,k);

    println!("Result: {}",result);
}

