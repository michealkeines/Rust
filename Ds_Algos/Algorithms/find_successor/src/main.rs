use std::rc::Rc;
use std::cell::RefCell;
use my_project::BT;
use my_project::Node;

type Link = Rc<RefCell<Node>>;

fn find_successor(node: Link) -> Option<Link> {
    let current: Link = node;
    if current.borrow().right.is_none() {
        return climbup(current);
    } else {
        return climbdown(current);
    }
}

fn climbup(current: Link) -> Option<Link> {
    if !current.borrow().parent.is_none() {
        if !current.borrow().parent.as_ref().unwrap().borrow().left.is_none() && current.borrow().parent.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().value == current.borrow().value {
            return current.borrow().parent.to_owned();
        }
        return climbup(current.borrow().parent.to_owned().unwrap());
    }
    None
}

fn climbdown(current: Link) -> Option<Link> {
    let current = current.borrow().right.to_owned();
    if !current.is_none() {
        if current.as_ref().unwrap().borrow().left.is_none() {
            return current;
        }
        return climbdown(current.as_ref().unwrap().borrow().left.to_owned().unwrap());
    }
    None
}

fn main() {
    // Initialize The Input Binary Tree
    let mut bt = BT::new();
    let node1 = Node::new(1);
    let node2 = Node::new(2);
    let node3 = Node::new(3);
    let node4 = Node::new(4);
    let node5 = Node::new(5);
    let node6 = Node::new(6);
    node4.borrow_mut().insert(Some(node2.clone()), Some(node6.clone()), None);
    node6.borrow_mut().insert(Some(node4.clone()), None, None);
    node5.borrow_mut().insert(Some(node2.clone()), None, None);
    node3.borrow_mut().insert(Some(node1.clone()), None, None);
    node2.borrow_mut().insert(Some(node1.clone()), Some(node4.clone()), Some(node5.clone()));
    node1.borrow_mut().insert(None, Some(node2.clone()), Some(node3.clone()));
    bt.root = Some(node1.clone());
    bt.root.unwrap().borrow().print();
    println!("\n");
    if let Some(result) = find_successor(node5.clone()) {
        println!("Successor of {} is {}.",result.borrow().value,node5.borrow().value);
    } else {
        println!("None");
    } 
}
