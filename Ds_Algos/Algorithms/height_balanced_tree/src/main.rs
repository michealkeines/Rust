use std::rc::Rc;
use std::cell::RefCell;
use my_project::BT;
use my_project::Node;
use std::cmp;

type Link = Rc<RefCell<Node>>;

struct Check {
    height: i32
}

impl Check {
    fn new(value: i32) -> Check {
        Check { height: value }
    }
}

fn is_balanced_binary_tree(tree: Link) {
    if let Some(_result) = helper(Some(&tree)) {
        println!("Tree is Balanced.");
    } else {
        println!("Tree is not Balanced");
    }
}

fn helper(tree: Option<&Link>) -> Option<Check> {
    if tree.is_none() { return Some(Check::new(0)); }

    let current = tree.unwrap();

    let left = helper(current.borrow().left.as_ref());
    let right = helper(current.borrow().right.as_ref());

    if left.is_none() && right.is_none() { return None; }

    let l: i32 = left.unwrap().height;
    let r: i32 = right.unwrap().height;

    let current_height: i32 = 1 + cmp::max(l,r);
    if cmp::max(l,r) - cmp::min(l,r) <= 1 { return Some(Check::new(current_height)); }
    else { return None }
}

fn main() {
    // Initialize The Input Binary Tree
    let mut bt = BT::new();
    let node1: Link = Node::new(1);
    let node2 = Node::new(2);
    let node3 = Node::new(3);
    let node4 = Node::new(4);
    let node5 = Node::new(5);
    let node6 = Node::new(6);
    let node7 = Node::new(7);
    let node8 = Node::new(8);
    
    node5.borrow_mut().insert(Some(node7), Some(node8));
    node3.borrow_mut().insert(None, Some(node6));
    node2.borrow_mut().insert(Some(node4), Some(node5));
    node1.borrow_mut().insert(Some(node2), Some(node3));
    
    bt.root = Some(node1.clone());
    bt.root.unwrap().borrow().print();
    println!("\n");
    is_balanced_binary_tree(node1);
}
