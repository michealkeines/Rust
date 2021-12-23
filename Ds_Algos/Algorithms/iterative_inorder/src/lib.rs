use std::rc::Rc;
use std::cell::RefCell;

pub struct BT {
    pub root: Link
}

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Clone, Debug)]
pub struct Node {
    pub value: i32,
    pub left: Link,
    pub right: Link,
    pub parent: Link
}

impl BT {
    pub fn new() -> Self {
        BT { root: None }
    }
}

impl Node {
    pub fn new(value: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node { value: value, left: None, right: None, parent: None }))
    }
}

impl Node {
    pub fn insert(&mut self, left: Link,right: Link, parent: Link) {
        self.left = left;
        self.right = right;
        self.parent = parent;
    }
    pub fn print(&self) {
        if !self.left.is_none() {
            self.left.as_ref().unwrap().borrow().print();
        }
        print!("{}, ",self.value);
        if !self.right.is_none() {
            self.right.as_ref().unwrap().borrow().print();
        }
    }

    pub fn is_equal(&self, node: &Link) -> bool {
        if let Some(val) = node {
            if self.value == val.borrow().value {
                return true;
            } else { return false; }
        } 
        else { return false; }     
    }

    
}

impl BT {
    pub fn traverse(&self) {
        self.root.as_ref().unwrap().borrow().print();
    }
}



