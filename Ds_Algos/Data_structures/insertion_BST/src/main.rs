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

    println!("Root Node: {:?}",bst.borrow().value);

    println!("Left: {:?}",bst.borrow().left.as_ref().unwrap().borrow().value);
    println!("Right: {:?}",bst.borrow().right.as_ref().unwrap().borrow().value);

    println!("Left of Left: {:?}",bst.borrow().left.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().value);
    println!("Left of Right: {:?}",bst.borrow().left.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().value);
    println!("Right of Right: {:?}",bst.borrow().right.as_ref().unwrap().borrow().right.as_ref().unwrap().borrow().value); 
    println!("Right of Left: {:?}",bst.borrow().right.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().value);
}



