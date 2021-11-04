use std::rc::Rc;
use std::cell::RefCell;
use my_project::BST;

type Node = Rc<RefCell<BST>>;

// #[derive(Clone)]
// pub struct BST {
//     pub value: i32,
//     pub left:  Option<Rc<RefCell<BST>>>,
//     pub right: Option<Rc<RefCell<BST>>>
// }

// impl BST {
//     pub fn new(val: i32) -> Node {
//         Rc::new(RefCell::new(BST { value: val, left: None, right: None }))
//     }
//     pub fn new_with_nodes(val:i32, left: Option<Node>, right: Option<Node>) -> Node {
//         Rc::new(RefCell::new(BST { value: val, left: left, right: right }))
//     }
// }

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

struct Index {
    value: usize
}

fn construct_bst(arr: & Vec<i32>) -> Node {
    let mut index = Index { value: 0 };
    helper(i32::MIN, i32::MAX, &mut index, arr).unwrap()
}

fn helper(lower: i32, upper: i32, index: &mut Index, arr: &Vec<i32>) -> Option<Node> {
    if index.value == arr.len() { return None; }
    
    let current: i32 = arr[index.value];
    if current < lower || current >= upper { return None; }

    index.value += 1;
    let left: Option<Node> = helper(lower,current,index,arr);
    let right: Option<Node> = helper(current,upper,index,arr);

    Some(BST::new_with_nodes(current,left,right))
}


fn main() {
    let arr: Vec<i32> = vec![10, 4, 2, 1, 5, 17, 19, 18];

    let result: Node = construct_bst(&arr);
    println!("Result: {:?}",preorder_traverse(result));
}

