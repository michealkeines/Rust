use std::rc::Rc;
use std::cell::RefCell;
use std::mem;

type BareTree = Rc<RefCell<Node>>;
type Tree = Option<BareTree>;

#[derive(Clone, Debug)]
pub struct IoTDevice {
    pub numerical_id: u64,
    pub address: String
}

#[derive(Clone, Debug, PartialEq)]
enum Color {
    Red,Black
}

#[derive(PartialEq)]
enum RBOperation {
    LeftNode,
    RightNode
}

struct Node {
    pub color: Color,
    pub dev: IoTDevice,
    pub parent: Tree,
    left: Tree,
    right: Tree
}

struct RBTree {
    root: Option<BareTree>,
    length: u64
}

impl RBTree {
    pub fn add(&mut self, device: IoTDevice) {
        self.length += 1;
        let root = mem::replace(&mut self.root, None);
        let new_tree = self.add_r(root, device);
        self.root = self.fix_tree(new_tree.1);
    }

    fn add_r(&mut self, mut node: Tree, device: IoTDevice) -> (Tree, BareTree) {
        if let Some(n) = node.take() {
            let new: BareTree;
            let current_device = n.borrow().dev.clone();

            match self.check(&current_device, &device) {
                RBOperation::LeftNode => {
                    let left = n.borrow().left.clone();
                    let new_tree = self.add_r(left, device);
                    new = new_tree.1;
                    let new_tree = new_tree.0.unwrap();
                    new_tree.borrow_mut().parent = Some(n.clone());
                    n.borrow_mut().left = Some(new_tree);
                },
                RBOperation::RightNode => {
                    let right = n.borrow().right.clone();
                    let new_tree = self.add_r(right, device);
                    new = new_tree.1;
                    let new_tree = new_tree.0.unwrap();
                    new_tree.borrow_mut().parent = Some(n.clone());
                    n.borrow_mut().right = Some(new_tree);
                }
            }
            (Some(n), new)
        } else {
            let new = Node::new(device);
            (new.clone(), new.unwrap())
        }
    }

    fn check(&self, a: &IoTDevice, b:&IoTDevice) -> RBOperation {
        if a.numerical_id <= b.numerical_id {
            RBOperation::LeftNode
        } else {
            RBOperation::RightNode
        }
    }

    fn fix_tree(&mut self, inserted: BareTree) -> Tree {
        let mut not_root = inserted.borrow().parent.is_some();

        let root = if not_root {
            let mut parent_is_red = self.parent_color(&inserted) == Color::Red;
            let mut n = inserted.clone();

            while parent_is_red && not_root {
                if let Some(uncle) = self.uncle(n.clone()) {
                    let which_side = uncle.1;
                    let uncle = uncle.0;

                    match which_side {
                        RBOperation::LeftNode => {
                            let mut parent = n.borrow().parent.as_ref().unwrap().clone();

                            if uncle.is_some() && uncle.as_ref().unwrap().borrow().color == Color::Red {
                                let uncle = uncle.unwrap();
                                parent.borrow_mut().color = Color::Black;
                                uncle.borrow_mut().color = Color::Black;
                                parent.borrow().parent.as_ref().unwrap().borrow_mut().color = Color::Red;
                                n = parent.borrow().parent.as_ref().unwrap().clone();
                            } else {
                                if self.check(&parent.borrow().dev, &n.borrow().dev) == RBOperation::LeftNode {
                                    let tmp = n.borrow().parent.as_ref().unwrap().clone();
                                    n = tmp;
                                    self.rotate(n.clone(), Rotation::Right);
                                    parent = n.borrow().parent.as_ref().unwrap().clone();
                                }
                                parent.borrow_mut().color = Color::Black;
                                parent.borrow().parent.as_ref().unwrap().borrow_mut().color = Color::Red;
                                let grandparent = n.borrow().parent.as_ref().unwrap().borrow().parent.as_ref().unwrap().clone();
                                self.rotate(grandparent, Rotation::Left);
                            }
                            
                        },
                        RBOperation::RightNode => {

                        }
                    }
                }
            }
        }
    }
    pub fn find(&self, numerical_id: u64) -> Option<IoTDevice> {
        self.find_r( &self.root, &IoTDevice::new(numerical_id,
        "".to_owned(), "".to_owned()), )}fn find_r(&self, node:
        &Tree, dev: &IoTDevice) -> Option<IoTDevice> { match node {
        Some(n) => { let n = n.borrow(); if n.dev.numerical_id ==
        dev.numerical_id { Some(n.dev.clone()) } else { match
        self.check(&n.dev, &dev) { RBOperation::LeftNode => self.find_r(&n.left,
        dev), RBOperation::RightNode => self.find_r(&n.right, dev), }
        } } _ => None, }}
        pub fn walk(&self, callback: impl Fn(&IoTDevice) -> ()) {
        self.walk_in_order(&self.root, &callback);}fn
        walk_in_order(&self, node: &Tree, callback: &impl Fn(&IoTDevice) -> ())
        { if let Some(n) = node { let n = n.borrow();
        self.walk_in_order(&n.left, callback); callback(&n.dev);
        self.walk_in_order(&n.right, callback); }}
}

impl Node {
    pub fn new(dev: IoTDevice) -> Tree {
        Some(
            Rc::new(
                RefCell::new(
                    Node {
                        color: Color::Red,
                        dev: dev,
                        parent: None,
                        left: None,
                        right: None
                    }
                )
            )
        )
    }
}

