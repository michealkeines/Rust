
use my_project::BST;
use std::rc::Rc;
use std::cell::RefCell;

type Node = Rc<RefCell<BST>>;

fn inorder_traverse(tree: Node) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    inorder(Some(tree), &mut result);
    result
}

fn inorder(tree: Option<Node>, result: &mut Vec<i32>) -> Option<bool> {
    if tree.is_none() { return None } // Return None if we reach a None value

    let current_tree = tree.unwrap();
    let current_value = current_tree.borrow().value;

    inorder(current_tree.to_owned().borrow().left.to_owned(),result);
    result.push(current_value);
    inorder(current_tree.to_owned().borrow().right.to_owned(),result);

    Some(true) // This return doesn't matter but to maintain same return Type, Rust Specific Impl
}

fn preorder_traverse(tree: Node) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    preorder(Some(tree), &mut result);
    result
}

fn preorder(tree: Option<Node>, result: &mut Vec<i32>) -> Option<bool> {
    if tree.is_none() { return None }

    let current_tree = tree.unwrap();
    let current_value = current_tree.borrow().value;

    result.push(current_value);
    preorder(current_tree.to_owned().borrow().left.to_owned(),result);
    preorder(current_tree.to_owned().borrow().right.to_owned(),result);

    Some(true)
}

fn postorder_traverse(tree: Node) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    postorder(Some(tree), &mut result);
    result
}

fn postorder(tree: Option<Node>, result: &mut Vec<i32>) -> Option<bool> {
    if tree.is_none() { return None }

    let current_tree = tree.unwrap();
    let current_value = current_tree.borrow().value;

    postorder(current_tree.to_owned().borrow().left.to_owned(),result);
    postorder(current_tree.to_owned().borrow().right.to_owned(),result);
    result.push(current_value);

    Some(true)
}



fn main() {
    let bst = BST::new(10); // Root Node

    BST::insert(bst.clone(),  5);
    BST::insert(bst.clone(), 15);
    BST::insert(bst.clone(),  2);
    BST::insert(bst.clone(),  5);
    BST::insert(bst.clone(), 22);
    BST::insert(bst.clone(),  1);


    //          10
    //        /    \
    //       5      15
    //     /   \      \
    //    2     5      22
    //   /
    //  1

    let inorder: Vec<i32> = inorder_traverse(bst.clone());
    let preorder: Vec<i32> = preorder_traverse(bst.clone());
    let postorder: Vec<i32> = postorder_traverse(bst.clone());
    println!("Inorder: {:?} \nPreorder: {:?} \nPostorder: {:?}",inorder, preorder, postorder);
}
