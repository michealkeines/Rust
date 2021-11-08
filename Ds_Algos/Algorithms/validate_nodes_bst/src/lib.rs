use std::rc::Rc;
use std::cell::RefCell;

type Node = Rc<RefCell<BST>>;

#[derive(Clone)]
pub struct BST {
    pub value: i32,
    pub left:  Option<Rc<RefCell<BST>>>,
    pub right: Option<Rc<RefCell<BST>>>
}

impl BST {
    pub fn new(val: i32) -> Node {
        Rc::new(RefCell::new(BST { value: val, left: None, right: None }))
    }
    pub fn new_with_nodes(val:i32, left: Option<Node>, right: Option<Node>) -> Node {
        Rc::new(RefCell::new(BST { value: val, left: left, right: right }))
    }
}

impl BST {
    pub fn insert(bst: Node, value: i32) -> Node {
        let mut current = bst;

        loop {
            let v = current.borrow_mut().value.clone(); 
            if v > value {
                if !current.borrow().left.is_none() {
                    current = current.to_owned().borrow().left.clone().unwrap();
                } else {
                    current.borrow_mut().left = Some(BST::new(value));
                    break;
                }
            } else if v <= value {
                if !current.borrow().right.is_none() {
                    current = current.to_owned().borrow().right.clone().unwrap();
                } else {
                    current.borrow_mut().right = Some(BST::new(value));
                    break;
                }
            }
        }
        return current
    }

    pub fn contains(bst: Node, value: i32) -> bool {
        let mut current = bst;
        loop {
            if current.borrow().value == value { return true; }
            else if current.borrow().value > value {
                if !current.borrow().left.is_none() {
                    current = current.to_owned().borrow().left.clone().unwrap();
                } else {
                    return false;
                }
            } else if current.borrow().value < value {
                if !current.borrow().right.is_none() {
                    current = current.to_owned().borrow().right.clone().unwrap();
                } else {
                    return false;
                }
            } 
        }
    }

    pub fn contains_return(bst: Rc<RefCell<BST>>, value: i32) -> Option<Node> {
        let mut current = bst;
        loop {
            if current.borrow().value == value { return Some(current.to_owned()); }
            else if current.borrow().value > value {
                if !current.borrow().left.is_none() {
                    current = current.to_owned().borrow().left.clone().unwrap();
                } else {
                    return None;
                }
            } else if current.borrow().value < value {
                if !current.borrow().right.is_none() {
                    current = current.to_owned().borrow().right.clone().unwrap();
                } else {
                    return None;
                }
            } 
        }
    }
}
