use std::rc::Rc;
use std::cell::RefCell;
use my_project::BT;
use std::mem;

type Node = Rc<RefCell<BT>>;

fn invert_bt(bt: &mut Node) {
    helper(&mut Some(bt));
}

fn helper(node: &mut Option<&mut Node>) {
    let current = node.as_mut().unwrap();
    swap(current);
    if !current.to_owned().borrow_mut().root.left.as_mut().is_none() {
        helper(&mut current.to_owned().borrow_mut().root.left.as_mut());
    }
    if !current.to_owned().borrow_mut().root.right.as_mut().is_none() {
        helper(&mut current.to_owned().borrow_mut().root.right.as_mut());
    }
}

fn swap(root: &mut Node) {
    let mut _temp = root.borrow_mut().root.left.to_owned();
    let mut _temp2 = root.borrow_mut().root.right.to_owned();
     let _v1 = mem::replace(&mut root.borrow_mut().root.left, _temp2);
     let _v2 = mem::replace(&mut root.borrow_mut().root.right, _temp);
}


fn main() {
    // Initialize The Input Binary Tree
    let child4: Node = BT::new(5);
    let child3: Node = BT::new(4);
    child3.borrow_mut().insert(Some(8),Some(9));
    let child2: Node = BT::new(3);
    child2.borrow_mut().insert(Some(6),Some(7));
    let child1: Node = BT::new(2);
    child1.borrow_mut().root.left= Some(child3);
    child1.borrow_mut().root.right = Some(child4);
    let mut bt: Node = BT::new(1);
    bt.borrow_mut().root.left= Some(child1);
    bt.borrow_mut().root.right= Some(child2);
    println!("Initial: ");
    bt.borrow_mut().print();
    println!("");
    invert_bt(&mut bt);
    println!("After: ");
    bt.borrow_mut().print();
    println!("");
}

