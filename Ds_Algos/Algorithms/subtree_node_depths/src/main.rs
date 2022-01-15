use my_project::BT;
use my_project::Node;
use std::cell::RefCell;
use std::rc::Rc;

type Link = Rc<RefCell<Node>>;

struct Tree_Info {
    num_nodes_in_tree: u32,
    sum_of_depths: u32,
    sum_of_all_depths: u32
}

impl Tree_Info {
    pub fn new(num_nodes_in_tree: u32, sum_of_depths: u32, sum_of_all_depths: u32) -> Tree_Info {
        Tree_Info {
            num_nodes_in_tree,
            sum_of_depths,
            sum_of_all_depths
        }
    }
}

fn all_kinds_of_node_depths(root: Link) -> u32 {
    helper(Some(root)).sum_of_all_depths
}

fn helper(node: Option<Link>) -> Tree_Info {
    
    if node.is_none() { return Tree_Info::new(0,0,0); }

    let current_node = node.unwrap();

    let left_tree_info = helper(current_node.borrow().left.clone());
    let right_tree_info = helper(current_node.borrow().right.clone());

    let sum_of_left_depths = left_tree_info.sum_of_depths + left_tree_info.num_nodes_in_tree;
    let sum_of_right_depths = right_tree_info.sum_of_depths + right_tree_info.num_nodes_in_tree;

    let num_nodes_in_tree = 1 + left_tree_info.num_nodes_in_tree + right_tree_info.num_nodes_in_tree;
    let sum_of_depths = sum_of_left_depths + sum_of_right_depths;
    let sum_of_all_depths = sum_of_depths + left_tree_info.sum_of_all_depths + right_tree_info.sum_of_all_depths;

    return Tree_Info::new(num_nodes_in_tree, sum_of_depths, sum_of_all_depths);
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
    let node8 = Node::new(8);
    let node9 = Node::new(9);

    node8.borrow_mut().insert(None, None);
    node9.borrow_mut().insert(None, None);
    node5.borrow_mut().insert(None, None);
    node6.borrow_mut().insert(None, None);
    node7.borrow_mut().insert(None, None);

    
    node4.borrow_mut().insert(Some(node8), Some(node9));
    node2.borrow_mut().insert(Some(node4), Some(node5));
    node3.borrow_mut().insert(Some(node6), Some(node7));
    node1.borrow_mut().insert(Some(node2), Some(node3));
    
    bt.root = Some(node1.clone());
    bt.root.unwrap().borrow().print();
    println!("\n");

    let result: u32 = all_kinds_of_node_depths(node1);
    println!("Result: {}",result);
}
