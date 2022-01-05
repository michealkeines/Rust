use my_project::BT;
use my_project::Node;
use std::cell::RefCell;
use std::rc::Rc;

type Link = Rc<RefCell<Node>>;

fn right_sibling_tree(root: Link) -> Link {
    helper(Some(root.clone()), None, false);
    root
}

fn helper(node: Option<Link>, parent: Option<Link>, is_left: bool) -> Option<Link> {
    if node.is_none() { return None; }

    let current: &Link = node.as_ref().unwrap();
    let left: Option<Link> = current.borrow_mut().left.clone();
    let right: Option<Link> = current.borrow_mut().right.clone();

    helper(left, node.clone(), true);
    let par: Link;
    if parent.is_none() {
        current.borrow_mut().right = None;
    } else if is_left == true {
        par = parent.unwrap();
        current.borrow_mut().right = par.borrow_mut().right.clone();
    } else {
        par = parent.unwrap();
        if !par.borrow().right.is_none() {
            current.borrow_mut().right = par.borrow_mut().right.as_ref().unwrap().borrow_mut().left.clone();
        } else {
            current.borrow_mut().right = None;
        }
    }
    helper(right, node.clone(), false)
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
    let node9 = Node::new(9);
    let node10 = Node::new(10);
    let node11 = Node::new(11);
    let node12 = Node::new(12);
    let node13 = Node::new(13);
    let node14 = Node::new(14);

    node8.borrow_mut().insert(None, None);
    node9.borrow_mut().insert(None, None);
    node10.borrow_mut().insert(None, None);
    node12.borrow_mut().insert(None, None);
    node13.borrow_mut().insert(None, None);
    node14.borrow_mut().insert(None, None);

    node11.borrow_mut().insert(Some(node14.clone()), None);
    node6.borrow_mut().insert(Some(node11.clone()), None);
    node5.borrow_mut().insert(None, Some(node10.clone()));
    
    node4.borrow_mut().insert(Some(node8), Some(node9));
    node7.borrow_mut().insert(Some(node12), Some(node13));
    node2.borrow_mut().insert(Some(node4), Some(node5));
    node3.borrow_mut().insert(Some(node6), Some(node7));
    node1.borrow_mut().insert(Some(node2), Some(node3));
    
    bt.root = Some(node1.clone());
    bt.root.unwrap().borrow().print();
    println!("\n");

    let result: Link = right_sibling_tree(node1);
    bt.root = Some(result);
    bt.root.unwrap().borrow().print();
    println!("\n");

}
