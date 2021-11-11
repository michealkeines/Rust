use std::rc::Rc;
use std::cell::RefCell;

type Node = Rc<RefCell<BST>>;

pub struct BST {
    pub root: NODE
}


pub struct NODE {
    pub value: i32,
    pub left: Option<Node>,
    pub right: Option<Node>,
    pub index: usize,
    pub totalatinsert: usize,
    pub leftsubtreetotal: usize
}

impl NODE {
    pub fn new(value: i32, totalatinsert: usize, index: usize) -> NODE {
        NODE {
            value: value,
            left: None,
            right: None,
            index: index,
            totalatinsert: totalatinsert,
            leftsubtreetotal: 0
        }        
    }
}

impl BST {
    pub fn new(value: i32, totalatinsert: usize, index: usize) -> Node {
        Rc::new(RefCell::new(BST { root: NODE::new(value, totalatinsert,index)}))
    }
}


impl BST {
    pub fn insert(&mut self, value: i32, totalatinsert: usize, index: usize) {
        let mut current = &mut self.root;
            if value < current.value {
                current.leftsubtreetotal += 1;
                if current.left.is_none() {
                    current.left = Some(BST::new(value,totalatinsert,index));
                } else {
                    current.left.to_owned().unwrap().borrow_mut().insert(value,totalatinsert,index);
                }    
            }
            else {
                let mut t = totalatinsert + current.leftsubtreetotal;
                if value > current.value {
                    t += 1;
                }
                if current.right.is_none() {
                    current.right = Some(BST::new(value, t,index));
                } else {
                    current.right.to_owned().unwrap().borrow_mut().insert(value, t,index);
                }
                
            }
    }

    pub fn update(&mut self, result: &mut Vec<i32>) { // Inorder Traverse to Print all Values
        let current = &mut self.root;
        if !current.left.is_none() {
            current.left.to_owned().unwrap().borrow_mut().update(result);
        }
        result[current.index] = current.totalatinsert as i32;
        if !current.right.is_none() {
            current.right.to_owned().unwrap().borrow_mut().update(result);
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

