use my_project::BT;
use my_project::Node;
use std::cell::RefCell;
use std::rc::Rc;

type Link = Rc<RefCell<Node>>;

fn leaf_traversal(tree1: Link, tree2: Link) -> bool {
    let (head1, _prev1) = connect_leaf(Some(tree1), None, None);
    let (head2, _prev2) = connect_leaf(Some(tree2), None, None);

    let mut currentnode1 = head1;
    let mut currentnode2 = head2;
    while !currentnode1.is_none() && !currentnode2.is_none() {
        let current1 = currentnode1.unwrap();
        let current2 = currentnode2.unwrap();

        println!("{},{}",current1.borrow().value,current2.borrow().value);

        if current1.borrow().value != current2.borrow().value {
            return false;
        } else {
            currentnode1 = current1.borrow().right.clone();
            currentnode2 = current2.borrow().right.clone();
        }
    }
    true
}

fn connect_leaf(tree: Option<Link>, head: Option<Link>, prev: Option<Link>) -> (Option<Link>, Option<Link>) {

    if tree.is_none() { return (head, prev); }

    let mut head = head;
    let mut prev = prev;

    let current = tree.unwrap();
    if current.borrow().left.is_none() && current.borrow().right.is_none() {
        if prev.is_none() {
            head = Some(current.clone());
        } else {
            prev.as_ref().unwrap().borrow_mut().right.replace(current.clone());
        }
        prev = Some(current.clone());
    }

    let (lefthead, leftprev) = connect_leaf(current.borrow().left.clone(), head, prev);
    return connect_leaf(current.borrow().right.clone(), lefthead, leftprev);


}



fn main() {
    // Initialize The Input Binary Tree
    let mut bt1 = BT::new();
    let tr1_node1: Link = Node::new(1);
    let tr1_node2 = Node::new(2);
    let tr1_node3 = Node::new(3);
    let tr1_node4 = Node::new(4);
    let tr1_node5 = Node::new(5);
    let tr1_node6 = Node::new(6);
    let tr1_node7 = Node::new(7);
    let tr1_node8 = Node::new(8);

    tr1_node4.borrow_mut().insert(None, None);
    tr1_node7.borrow_mut().insert(None, None);
    tr1_node8.borrow_mut().insert(None, None);
    tr1_node6.borrow_mut().insert(None, None);

    
    tr1_node5.borrow_mut().insert(Some(tr1_node7), Some(tr1_node8));
    tr1_node2.borrow_mut().insert(Some(tr1_node4), Some(tr1_node5));
    tr1_node3.borrow_mut().insert(None, Some(tr1_node6));
    tr1_node1.borrow_mut().insert(Some(tr1_node2), Some(tr1_node3));
    
    bt1.root = Some(tr1_node1.clone());
    bt1.root.unwrap().borrow().print();
    println!("\n");

    let mut bt2 = BT::new();
    let tr2_node1: Link = Node::new(1);
    let tr2_node2 = Node::new(2);
    let tr2_node3 = Node::new(3);
    let tr2_node4 = Node::new(4);
    let tr2_node5 = Node::new(5);
    let tr2_node6 = Node::new(6);
    let tr2_node7 = Node::new(7);
    let tr2_node8 = Node::new(8);

    tr2_node4.borrow_mut().insert(None, None);
    tr2_node7.borrow_mut().insert(None, None);
    tr2_node8.borrow_mut().insert(None, None);
    tr2_node6.borrow_mut().insert(None, None);

    
    tr2_node5.borrow_mut().insert(Some(tr2_node8), Some(tr2_node6));
    tr2_node2.borrow_mut().insert(Some(tr2_node4), Some(tr2_node7));
    tr2_node3.borrow_mut().insert(None, Some(tr2_node5));
    tr2_node1.borrow_mut().insert(Some(tr2_node2), Some(tr2_node3));
    
    bt2.root = Some(tr2_node1.clone());
    bt2.root.unwrap().borrow().print();
    println!("\n");

    let result: bool = leaf_traversal(tr1_node1,tr2_node1);
    println!("Result: {}",result);
}
