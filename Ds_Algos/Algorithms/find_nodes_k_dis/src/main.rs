use my_project::BT;
use my_project::Node;
use std::cell::RefCell;
use std::rc::Rc;

type Link = Rc<RefCell<Node>>;

fn nodes_with_distance_k(node: Link, target: i32, k: u32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    helper(Some(node), target, k, &mut result);
    result
}

fn helper(node: Option<Link>, target: i32, k: u32, result: &mut Vec<i32>) -> i32 {
    
    if node.is_none() { return -1; }

    let current_node = node.as_ref().unwrap();

    if current_node.borrow().value == target {
        add_value_with_distance_k(&node, 0, k, result);
        return 1;
    }

    let left = helper(current_node.borrow().left.clone(), target, k, result);
    let right = helper(current_node.borrow().right.clone(), target, k, result);

    if right == k as i32 || left == k as i32{
        result.push(current_node.borrow().value);
    }

    if left != -1 {
        add_value_with_distance_k(&current_node.borrow().right.clone(), left + 1, k, result);
        return left + 1;
    }

    if right != -1 {
        add_value_with_distance_k(&current_node.borrow().left.clone(), right + 1, k, result);
        return right + 1;
    }

    return -1;
}

fn add_value_with_distance_k(node: &Option<Link>, distance: i32, k: u32, result: &mut Vec<i32>) {
    if node.is_none() { return; }

    if distance == k.try_into().unwrap() {
        result.push(node.as_ref().unwrap().borrow().value);
    }
    add_value_with_distance_k(&node.as_ref().unwrap().borrow().left.clone(), distance + 1, k, result);
    add_value_with_distance_k(&node.as_ref().unwrap().borrow().right.clone(), distance + 1, k, result);
}

// Recursion cases

// it is in left subtree
// it is in right subtree
// it is the current node
// it is not in any subtree

// we will represent -1 as target node not in that subtree
// we will return 1 when we are at the target node, thus we know that we are one away from the caller.




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
    let node8 = Node::new(8);
    
    node6.borrow_mut().insert(Some(node7), Some(node8));
    node3.borrow_mut().insert(None, Some(node6));
    node2.borrow_mut().insert(Some(node4), Some(node5));
    node1.borrow_mut().insert(Some(node2), Some(node3));
    
    bt.root = Some(node1.clone());
    bt.root.unwrap().borrow().print();
    println!("\n");

    let target = 3;
    let k = 2;
    let result: Vec<i32> = nodes_with_distance_k(node1, target, k);
    println!("Result: {:?}", result);

}







