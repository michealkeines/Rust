use std::rc::Rc;
use std::cell::RefCell;
use my_project::BT;

type Node = Rc<RefCell<BT>>;

struct Sum {
    value : i32
}

fn nodes_depth_sum(bt: Node) -> i32 {
    let mut total: Sum = Sum { value: 0 };
    helper(&bt,0,&mut total);
    total.value
}

fn helper(node: &Node, depth: i32, total:&mut Sum) {
    total.value += depth;
    if !node.borrow().root.left.is_none() {
        helper(&node.borrow().root.left.to_owned().unwrap(), depth + 1, total);
    }
    if !node.borrow().root.right.is_none() {
        helper(&node.borrow().root.right.to_owned().unwrap(), depth + 1, total);
    }
}


fn main() {
    // Initialize The Input Binary Tree
    let child4: Node = BT::new(5);
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

    let result: i32 = nodes_depth_sum(bt);
    println!("Result: {}",result);
}

