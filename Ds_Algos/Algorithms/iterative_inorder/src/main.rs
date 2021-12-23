use my_project::BT;
use my_project::Node;
use std::cell::RefCell;
use std::rc::Rc;

type Link = Rc<RefCell<Node>>;

fn iterative_inorder(tree: Link) -> Vec<i32> {
    
    let mut current: Option<Link> = Some(tree);
    let mut prev: Option<Link> = None;
    let mut _next: Option<Link> = None;
    let mut result = vec![];

    while !current.as_ref().is_none() {

        if prev.as_ref().is_none() || prev.as_ref().unwrap().borrow().is_equal(&current.as_ref().unwrap().borrow().parent.clone()) {
            if !current.as_ref().unwrap().borrow().left.as_ref().is_none() {
                _next = current.as_ref().unwrap().borrow().left.clone();
            } else {
                result.push(current.as_ref().unwrap().borrow().value);
                _next = if let Some(val) = &current.as_ref().unwrap().borrow().right {
                    Some(val.clone())
                } else {
                    current.as_ref().unwrap().borrow().parent.clone()
                }
            }
        } 
        
        else if  prev.as_ref().unwrap().borrow().is_equal(&current.as_ref().unwrap().borrow().left.clone()) {
            result.push(current.as_ref().unwrap().borrow().value);
            _next = if let Some(val) = &current.as_ref().unwrap().borrow().right {
                        Some(val.clone())
                    } else {
                        current.as_ref().unwrap().borrow().parent.clone()
                    }
        } 
        
        else {
            _next = current.as_ref().unwrap().borrow().parent.clone()
        }
        prev = current.clone();
        current = _next.clone();
    }
    result
}


fn main() {
    // Initialize The Input Binary Tree
    let mut bt = BT::new();
    let node1: Link = Node::new(1);
    let node2 = Node::new(2);
    let node3 = Node::new(3);
    let node4 = Node::new(4);
    let node6 = Node::new(6);
    let node7 = Node::new(7);
    let node9 = Node::new(9);
    
    node6.borrow_mut().insert(None, None, Some(node3.clone()));
    node7.borrow_mut().insert(None, None, Some(node3.clone()));
    node9.borrow_mut().insert(None, None, Some(node4.clone()));
    node3.borrow_mut().insert(Some(node6), Some(node7), Some(node1.clone()));
    node2.borrow_mut().insert(Some(node4.clone()), None, Some(node1.clone()));
    node4.borrow_mut().insert(None, Some(node9.clone()), Some(node2.clone()));
    node1.borrow_mut().insert(Some(node2), Some(node3), None);
    
    bt.root = Some(node1.clone());
    bt.root.unwrap().borrow().print();
    println!("\n");

    let result: Vec<i32> = iterative_inorder(node1);
    println!("Result: {:?}", result);
//
}
