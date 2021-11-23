use std::rc::Rc;
use std::cell::RefCell;

type Node = Rc<RefCell<BT>>;

#[derive(Clone, Debug)]
pub struct BT {
    pub root: NODE
}

#[derive(Clone, Debug)]
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

impl BT {
    pub fn new(value: i32) -> Node {
        Rc::new(RefCell::new(BT { root: NODE::new(value)}))
    }
}

impl BT {
    
    pub fn insert(&mut self, left: Option<i32>, right: Option<i32>) {
        if !left.is_none() {
            self.root.left = Some(BT::new(left.unwrap()));
        } else {
            self.root.left = None;
        }
        if !right.is_none() {
            self.root.right = Some(BT::new(right.unwrap()));
        } else {
            self.root.right = None;
        }
        
    }

    pub fn print(&mut self) {
        print!("{}, ",self.root.value);
        if !self.root.left.is_none() {
            self.root.left.to_owned().unwrap().borrow_mut().print();
        }
        if !self.root.right.is_none() {
            self.root.right.to_owned().unwrap().borrow_mut().print();
        }
    }
}
