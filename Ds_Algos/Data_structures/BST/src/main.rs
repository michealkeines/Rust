use my_project::BST;

fn main() {
    let root = BST::new(10);
    root.borrow_mut().insert(5);
    root.borrow_mut().insert(15);
    root.borrow_mut().insert(2);
    root.borrow_mut().insert(5);
    root.borrow_mut().insert(1);
    root.borrow_mut().insert(22);
    
    root.borrow_mut().print();
}

