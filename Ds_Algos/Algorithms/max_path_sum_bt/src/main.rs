use std::rc::Rc;
use std::cell::RefCell;
use my_project::BT;
use my_project::Node;
use std::cmp;

type Link = Rc<RefCell<Node>>;

struct Check {
    max_sum_as_branch: i32,
    running_max_sum: i32
}

impl Check {
    fn new(val1: i32, val2: i32) -> Check {
        Check { max_sum_as_branch: val1, running_max_sum: val2 }
    }
}

fn max_path_sum(tree: Link) -> i32 {
    return helper(Some(&tree)).running_max_sum;
}

fn helper(node: Option<&Link>) -> Check {
    if node.is_none() { return Check::new(0,i32::MIN); }

    let current: &Link = node.unwrap();
    let left: Check = helper(current.as_ref().borrow().left.as_ref());
    let right: Check = helper(current.as_ref().borrow().right.as_ref());

    let left_max_sum_as_branch: i32 = left.max_sum_as_branch;
    let left_max_path_sum: i32 = left.running_max_sum;

    let right_max_sum_as_branch: i32 = right.max_sum_as_branch;
    let right_max_path_sum: i32 = right.running_max_sum;
    
    let max_child_sum_as_branch: i32 = cmp::max(left_max_sum_as_branch, right_max_path_sum);
    
    let value: i32 = current.borrow().value;

    let max_sum_as_branch: i32 = cmp::max(max_child_sum_as_branch + value, value);
    let max_sum_including_triangle_form: i32 = cmp::max(left_max_sum_as_branch + value + right_max_sum_as_branch, max_sum_as_branch);
    let running_max_sum: i32 = cmp::max(cmp::max(left_max_path_sum, right_max_path_sum), max_sum_including_triangle_form);

    return Check::new(max_sum_as_branch, running_max_sum);
}



fn main() {
    // Initialize The Input Binary Tree
    let mut bt = BT::new();
    let node1: Link = Node::new(1);
    let node2 = Node::new(2);
    let node3 = Node::new(3);
    let node4 = Node::new(4);
    let node5 = Node::new(5);
    let node6 = Node::new(6);
    let node7 = Node::new(7);
    
    node3.borrow_mut().insert(Some(node6), Some(node7));
    node2.borrow_mut().insert(Some(node4), Some(node5));
    node1.borrow_mut().insert(Some(node2), Some(node3));
    
    bt.root = Some(node1.clone());
    bt.root.unwrap().borrow().print();
    println!("\n");
    let result: i32 = max_path_sum(node1);
    println!("Result: {}", result);
}
