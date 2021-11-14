use std::rc::Rc;
use std::cell::RefCell;
use my_project::BT;

type Node = Rc<RefCell<BT>>;

fn branch_sums(root: Node) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    helper(&root, 0, &mut result);
    result
}

fn helper(root: &Node, current_sum: i32, result: &mut Vec<i32>) {
    let current_root: Node = root.to_owned().to_owned();
    let current_val: i32 = current_root.borrow().root.value;
    if current_root.borrow().root.left.is_none() && current_root.borrow().root.right.is_none() {
        result.push(current_sum + current_val);
    } else {
        if !current_root.borrow().root.left.is_none() {
            helper(&current_root.borrow().root.left.to_owned().unwrap(),current_sum+current_val, result);
        }
        if !current_root.borrow().root.right.is_none() {
            helper(&current_root.borrow().root.right.to_owned().unwrap(),current_sum+current_val, result);
        }
    }
}


fn main() {
    // Initialize The Input Binary Tree
    let child4: Node = BT::new(5);
    child4.borrow_mut().insert(Some(10),None);
    let child3: Node = BT::new(4);
    child3.borrow_mut().insert(Some(8),Some(9));
    let child2: Node = BT::new(3);
    child2.borrow_mut().insert(Some(6),Some(7));
    let child1: Node = BT::new(2);
    child1.borrow_mut().root.left= Some(child3);
    child1.borrow_mut().root.right = Some(child4);
    let bt: Node = BT::new(1);
    bt.borrow_mut().root.left= Some(child1);
    bt.borrow_mut().root.right= Some(child2);

    let result: Vec<i32> = branch_sums(bt);
    println!("Result: {:?}",result);
}

