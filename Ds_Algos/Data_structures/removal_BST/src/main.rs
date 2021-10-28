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

    fn remove(bst: Rc<RefCell<BST>>, value: i32, parent: Option<Rc<RefCell<BST>>>) {
        let mut current = bst;
        loop {

            if current.borrow().value == value { 
                if !current.borrow().left.is_none() && !current.borrow().right.is_none() { // Has Both Children Left & Right
                    let tmp = current.borrow().value;
                    let res = BST::get_min_val(current.to_owned().borrow().right.to_owned().unwrap());
                    current.borrow_mut().value = res;
                    BST::remove(current.to_owned().borrow_mut().right.to_owned().unwrap(), tmp, Some(current.to_owned()));
                } else if parent.is_none() {
                    if !current.borrow().left.is_none() { // Root Node with Left Child
                        current.borrow_mut().value = current.borrow().left.to_owned().unwrap().borrow().value;
                        current.borrow_mut().right = current.to_owned().borrow().left.to_owned().unwrap().to_owned().borrow().right.to_owned();
                        current.borrow_mut().left = current.to_owned().borrow().left.to_owned().unwrap().to_owned().borrow().left.to_owned();
                    } else if !current.borrow().right.is_none() { // Root Node with Right Child
                        current.borrow_mut().value = current.borrow().right.to_owned().unwrap().borrow().value;
                        current.borrow_mut().left = current.to_owned().borrow().right.to_owned().unwrap().to_owned().borrow().left.to_owned();
                        current.borrow_mut().right = current.to_owned().borrow().right.to_owned().unwrap().to_owned().borrow().right.to_owned();
                    } else {
                        break; // Root Node with No Children
                    }
                } else if parent.to_owned().unwrap().borrow().left.to_owned().unwrap().borrow().value == current.borrow().value {
                    parent.unwrap().to_owned().borrow_mut().left = if !current.borrow_mut().left.is_none() { // Only Left Child
                        current.to_owned().borrow().left.to_owned()
                    } else {
                        current.to_owned().borrow().right.to_owned()
                    }

                } else if parent.to_owned().unwrap().borrow().right.to_owned().unwrap().borrow().value == current.borrow().value {
                    parent.unwrap().to_owned().borrow_mut().right = if !current.borrow_mut().left.is_none() { // Only Right Child
                        current.to_owned().borrow().left.to_owned()
                    } else {
                        current.to_owned().borrow().right.to_owned()
                    }
                    
                }
                break;
            }



            else if current.borrow().value > value {
                if !current.borrow().left.is_none() {
                    current = current.to_owned().borrow().left.clone().unwrap();
                } else {
                    break;
                }
            } else if current.borrow().value < value {
                if !current.borrow().right.is_none() {
                    current = current.to_owned().borrow().right.clone().unwrap();
                } else {
                    break;
                }
            }
        }
    }


    

    fn get_min_val(right: Rc<RefCell<BST>>) -> i32 {
        let mut current = right;
        let mut res: i32 = current.borrow().value;
        while !current.borrow().left.is_none() {
            current = current.to_owned().borrow().left.to_owned().unwrap();
            res = current.borrow().value;
        }
        return res;
    }

}




fn main() {
    let bst = BST::new(15); // Root Node

    BST::insert(bst.clone(), 11);
    let parent1 = BST::insert(bst.clone(), 17);
    BST::insert(bst.clone(), 14);
    BST::insert(bst.clone(), 19);
    BST::insert(bst.clone(), 16);
    let parent2 = BST::insert(bst.clone(),  9);

    //        15
    //      /    \
    //    11      17
    //  /   \    /  \
    // 9    14  16   19

    //Test Cases: remove 17, remove 15, remove 9

    println!("17 in the Tree: {}",BST::contains(bst.clone(),17));
    BST::remove(bst.clone(),17, Some(bst.clone()));
    println!("17 in the Tree: {}",BST::contains(bst.clone(),17));

    println!("15 in the Tree: {}",BST::contains(bst.clone(),15));
    BST::remove(bst.clone(),15, Some(parent1));
    println!("15 in the Tree: {}",BST::contains(bst.clone(),15));

    println!("9 in the Tree: {}",BST::contains(bst.clone(),9));
    BST::remove(bst.clone(),9, Some(parent2));
    println!("9 in the Tree: {}",BST::contains(bst.clone(),9));

    println!("Current Root: {}",bst.borrow().value);

    //        16
    //      /    \
    //    11      19
    //  /   \    /  \
    // None 14 None None
}


