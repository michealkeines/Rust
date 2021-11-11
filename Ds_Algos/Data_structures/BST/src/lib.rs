use std::rc::Rc;
use std::cell::RefCell;

type Node = Rc<RefCell<BST>>;

pub struct BST {
    pub root: NODE
}


pub struct NODE {
    pub value: i32,
    pub left: Option<Node>,
    pub right: Option<Node>
}

impl NODE {
    pub fn new(value: i32) -> NODE {
        NODE {
            value: value,
            left: None,
            right: None
        }        
    }
}

impl BST {
    pub fn new(value: i32) -> Node {
        Rc::new(RefCell::new(BST { root: NODE::new(value)}))
    }
}


impl BST {
    pub fn insert(&mut self, value: i32) {
        let mut current = &mut self.root;
            if value < current.value {
                if current.left.is_none() {
                    current.left = Some(BST::new(value));
                } else {
                    current.left.to_owned().unwrap().borrow_mut().insert(value);
                }    
            }
            else {
                if current.right.is_none() {
                    current.right = Some(BST::new(value));
                } else {
                    current.right.to_owned().unwrap().borrow_mut().insert(value);
                }
                
            }
    }

    pub fn print(&mut self) { // Inorder Traverse to Print all Values
        let current = &mut self.root;
        if !current.left.is_none() {
            current.left.to_owned().unwrap().borrow_mut().print();
        }
        println!("{}",current.value);
        if !current.right.is_none() {
            current.right.to_owned().unwrap().borrow_mut().print();
        }
    }
}

