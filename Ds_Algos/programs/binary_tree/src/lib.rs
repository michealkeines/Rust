
use std::mem;

type Tree = Option<Box<Node>>;

#[derive(Clone, Debug)]
pub struct IoTDevice {
    pub numerical_id: u64,
    pub address: String
}

struct Node {
    pub dev: IoTDevice,
    left: Tree,
    right: Tree
}

impl Node {
    fn new(device: IoTDevice) -> Tree {
        Some(Box::new(Node {
            dev: device,
            left: None,
            right: None
        }))
    }
}

struct BinaryTree {
    root: Tree,
    pub length: u64
}

impl BinaryTree {
    pub fn add(&mut self, device: IoTDevice) {
        self.length += 1;
        let root = mem::replace(&mut self.root, None);
        self.root = self.add_rec(root, device);
    }

    pub fn add_rec(&mut self, node: Tree, device: IoTDevice) -> Tree {
        match node {
            Some(mut n) => {
                if n.dev.numerical_id <= device.numerical_id {
                    n.left = self.add_rec(n.left, device);
                    Some(n)
                } else {
                    n.right = self.add_rec(n.right, device);
                    Some(n)
                }
            },
            _ => Node::new(device)
        }
    }

    pub fn find(&mut self, numerical_id: u64) -> Option<IoTDevice> {
        self.find_r(&self.root, numerical_id)
    }

    pub fn find_r(&self, node: &Tree, numerical_id: u64) -> Option<IoTDevice> {
        match node {
            Some(n) => {
                if n.dev.numerical_id == numerical_id {
                    Some(n.dev.clone())
                } else if n.dev.numerical_id > numerical_id {
                    self.find_r(&n.left, numerical_id)
                } else {
                    self.find_r(&n.right, numerical_id)
                }
            },
            _ => None
        }        
    }

    pub fn walk(&self, callback: &impl Fn(&IoTDevice)) {
        self.walk_in_order(&self.root, &callback);
    }

    pub fn walk_in_order(&self, node: &Tree, callback: &impl Fn(&IoTDevice)) {
        if let Some(n) = node {
            self.walk_in_order(&n.left, callback);
            callback(&n.dev);
            self.walk_in_order(&n.right, callback);
        }
    }
}