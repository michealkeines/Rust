
use std::rc::Rc;
use std::cell::RefCell;

type Node = Rc<RefCell<BST>>;

struct BST {
    value: i32,
    left: Option<Node>,
    right: Option<Node>
}

impl BST {
    pub fn new(value: i32) -> Node {
        Rc::new(RefCell::new(BST{value:value,left:None, right:None}))
    }
}

fn inorder_traverse(tree: Node) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    inorder(Some(tree), &mut result);
    result
}

fn inorder(tree: Option<Node>, result: &mut Vec<i32>) -> Option<bool> {
    if tree.is_none() { return None }

    let current_tree = tree.unwrap();
    let current_value = current_tree.borrow().value;

    inorder(current_tree.to_owned().borrow().left.to_owned(),result);
    result.push(current_value);
    inorder(current_tree.to_owned().borrow().right.to_owned(),result);

    Some(true)
}

fn min_height_bst(arr: Vec<i32>) -> Node {
    return helper(0,arr.len()-1,&arr).unwrap();
}

fn helper(start: usize, end: usize, arr: &Vec<i32>) -> Option<Node> {
    if end < start { return None; }

    let middle: usize = (end + start) / 2;
    let bst = BST::new(arr[middle]);

    bst.to_owned().borrow_mut().left = match middle {
       0 => {None},
       _ => {helper(start,middle-1,arr)}
    };
    bst.to_owned().borrow_mut().right = helper(middle+1,end,arr);

    return Some(bst);
}



fn main() {
    let arr: Vec<i32> = vec![1, 2, 5, 7, 10, 13, 14, 15, 22];

    let bst = min_height_bst(arr);
    let root: i32 = bst.borrow().value;
    println!("Root: {}",root);

    let inorder: Vec<i32> = inorder_traverse(bst);
    println!("{:?}",inorder);
}



