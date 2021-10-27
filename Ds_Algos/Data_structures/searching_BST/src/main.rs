use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub struct BST {
    value: i32,
    left:  Option<Rc<RefCell<BST>>>,
    right: Option<Rc<RefCell<BST>>>
}

impl BST {
    fn new(val: i32) -> Rc<RefCell<BST>> {
        Rc::new(RefCell::new(BST { value: val, left: None, right: None }))
    }
}

impl BST {
    pub fn insert(bst: Rc<RefCell<BST>>, value: i32) -> Rc<RefCell<BST>> {
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

    fn contains(bst: Rc<RefCell<BST>>, value: i32) -> bool {
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

}




fn main() {
    let bst = BST::new(15); // Root Node

    BST::insert(bst.clone(), 11);
    BST::insert(bst.clone(), 17);
    BST::insert(bst.clone(), 14);
    BST::insert(bst.clone(), 19);
    BST::insert(bst.clone(), 16);
    BST::insert(bst.clone(),  9);

    //        15
    //      /    \
    //    11      17
    //  /   \    /  \
    // 9    14  16   19

    println!("19 in the Tree: {}",BST::contains(bst.clone(),19));
    println!("13 in the Tree: {}",BST::contains(bst.clone(),13));

}
