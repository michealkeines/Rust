use std::rc::Rc;
use std::cell::RefCell;
use my_project::BST;

type Node = Rc<RefCell<BST>>;

fn validate_bst(tree: Node) -> bool {
    is_valid(Some(tree), i32::MIN, i32::MAX)
}

fn is_valid(tree: Option<Node>,min_val: i32,max_val: i32) -> bool {
    if tree.is_none() { return true }

    let current_tree: Node = tree.unwrap();
    let current_value: i32 = current_tree.borrow().value;
    if current_value < min_val || current_value >= max_val { return false }

    let left_child_is_valid: bool = is_valid(current_tree.to_owned().borrow().left.to_owned(), min_val, current_value);
    let right_child_is_valid: bool = is_valid(current_tree.to_owned().borrow().right.to_owned(), current_value, max_val);

    return left_child_is_valid && right_child_is_valid;
}

fn main() {
    let bst = BST::new(15); // Root Node

    BST::insert(bst.clone(), 11);
    BST::insert(bst.clone(), 17);
    BST::insert(bst.clone(), 14);
    BST::insert(bst.clone(), 19);
    BST::insert(bst.clone(), 16);
    BST::insert(bst.clone(),  9);

    //          15
    //        /    \
    //       5       15
    //     /   \    /  \
    //    2     5  13   22
    //   /           \
    //  1            14

    let result: bool = validate_bst(bst);
    println!("Result, Valid BST: {}",result);

    let bst = BST::new(15); // Root Node

    BST::insert(bst.clone(), 11);
    BST::insert(bst.clone(), 17);
    BST::insert(bst.clone(), 14);
    BST::insert(bst.clone(), 19);
    BST::insert(bst.clone(), 16);
    BST::insert(bst.clone(),  9);

    bst.borrow_mut().left.as_mut().unwrap().borrow_mut().right.as_mut().unwrap().borrow_mut().value = 32;

    println!("Invalid value added to Root.left.right.value: {}",bst.borrow().left.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().value);

    //          15
    //        /    \
    //       5      15
    //     /   \   /  \
    //    2    32 13   22
    //   /          \
    //  1           14  This is not a Valid BST now as 32 is left of root 15


    let result: bool = validate_bst(bst);
    println!("Result, Invalid BST: {}",result);
}

