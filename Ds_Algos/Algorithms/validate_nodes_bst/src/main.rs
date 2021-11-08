use my_project::BST;
use std::rc::Rc;
use std::cell::RefCell;

type Node = Rc<RefCell<BST>>;

fn validate_nodes(node1: Node, node2: Node, node3: Node) -> bool {
    if is_des(Some(node2.clone()),Some(node1.clone())) {
        return is_des(Some(node3.clone()),Some(node2.clone()));
    }
    if is_des(Some(node2.clone()), Some(node3.clone())) {
        return is_des(Some(node1.clone()), Some(node2.clone()));
    }
    return false;
}

fn is_des(node: Option<Node>, target: Option<Node>) -> bool {
    if node.is_none() { return false; }

    let temp_node = node.unwrap();
    let temp_target = target.unwrap();

    let node_val = temp_node.borrow().value;
    let target_val = temp_target.borrow().value;
    
    if node_val == target_val { return true; }

    if node_val > target_val {
        return is_des(temp_node.to_owned().borrow().left.to_owned(), Some(temp_target.to_owned()));
    } else {
        return is_des(temp_node.to_owned().borrow().right.to_owned(), Some(temp_target.to_owned()));
    }
}


fn main() {
    let bst = BST::new(5); // Root Node
    BST::insert(bst.clone(),  2);
    BST::insert(bst.clone(),  7);
    BST::insert(bst.clone(),  1);
    BST::insert(bst.clone(),  4);  
    BST::insert(bst.clone(),  0);
    BST::insert(bst.clone(),  6);
    BST::insert(bst.clone(),  8);
    BST::insert(bst.clone(),  3);

    let node1 = BST::contains_return(bst.clone(),5);
    let node2 = BST::contains_return(bst.clone(),2);
    let node3 = BST::contains_return(bst.clone(),3);

    //          5
    //        /    \
    //       2      7
    //     /   \   /  \
    //    1     4 6    8
    //   /     /          
    //  0     3

    let result: bool = validate_nodes(node1.unwrap(), node2.unwrap(), node3.unwrap());
    println!("Result: {}", result);
}

