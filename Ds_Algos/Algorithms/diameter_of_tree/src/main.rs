use std::rc::Rc;
use std::cell::RefCell;
use my_project::BT;
use std::cmp;

type Node = Rc<RefCell<BT>>;

struct Check {
    height: i32,
    diameter: i32
}

impl Check {
    fn new(height: i32, diameter: i32) -> Check {
        Check {
            height: height,
            diameter: diameter
        }
    }
}

fn get_diameter(bt: Node) -> i32 {
    helper(Some(bt)).diameter
}

fn helper(bt: Option<Node>) -> Check {
    if bt.is_none() { return Check::new(0,0); }

    let left: Check = helper(bt.to_owned().unwrap().borrow().root.left.to_owned());
    let right: Check = helper(bt.to_owned().unwrap().borrow().root.right.to_owned());

    let longest_path: i32 = left.height + right.height;
    let max_diameter: i32 = cmp::max(left.diameter, right.diameter);
    let current_diameter: i32 = cmp::max(longest_path, max_diameter);
    let current_height: i32 = 1 + cmp::max(left.height, right.height);

    return Check::new(current_height, current_diameter);
}


fn main() {
    // Initialize The Input Binary Tree
    let child9: Node = BT::new(9);
    let child8: Node = BT::new(8);
    let child7: Node = BT::new(7);
    let child6: Node = BT::new(6);
    let child5: Node = BT::new(5);
    let child4: Node = BT::new(4);
    let child3: Node = BT::new(3);
    let child2: Node = BT::new(2);
    child8.borrow_mut().root.left = Some(child9);
    child7.borrow_mut().root.left = Some(child8);
    child3.borrow_mut().root.left = Some(child7);
    child5.borrow_mut().root.right = Some(child6);
    child4.borrow_mut().root.right = Some(child5);
    child3.borrow_mut().root.right = Some(child4);
    let bt: Node = BT::new(1);
    bt.borrow_mut().root.left= Some(child3);
    bt.borrow_mut().root.right= Some(child2);

    let result: i32 = get_diameter(bt);
    println!("Result: {}",result);
    
}

