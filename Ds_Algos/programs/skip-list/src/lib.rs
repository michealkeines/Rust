use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    next: Vec<Link>,
    pub index: u64,
    pub value: String
}

pub struct Log {
    head: Link,
    tails: Vec<Link>,
    max_level: usize,
    pub length: u64
}

impl Node {
    fn new(list: Vec<Link>, index: u64, value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { next: list, index: index, value: value}))
    }
}

impl Log {
    pub fn push(&mut self, index: u64, value: String) {
        let level = 1 + if self.head.is_none() {
            self.max_level
        } else {
            self.get_level()
        };

        let node = Node::new(vec![None; level], index, value);

        for i in 0..level {
            if let Some(old) = self.tails[i].take() {
                let next = &mut old.borrow_mut().next;
                next[i] = Some(node.clone());
            }
            self.tails[i] = Some(node.clone());
        }

        if self.head.is_none() {
            self.head = Some(node.clone());
        }

        self.length += 1;
    }

    pub find(&self, offset: u64) -> Option<String> {
        match self.head {
            Some(ref head) => {
                let mut start_level = self.max_level;
                let node = head.clone();
                let mut result = None;

                loop {
                    if node.borrow().next[start_level].is_some() {
                        break;
                    }
                    start_level -= 1;
                }

                let mut n = node;
                for level in (0..=start_level).rev() {
                    loop {
                        let next = n.clone();
                        match next.borrow().next[level] {
                            Some(ref next)
                                if next.borrow().offset <= offset => n = next.clone(),
                            _ => break
                        };
                    }
                    if n.borrow().offset == offset {
                        let tmp = n.borrow();
                        result = Some(tmp.value.clone());
                        break;
                    }
                
                }
                result
            }
            None => None
        }
    }
    pub fn new(max: u64) -> Log {
        Log {
            head: None,
            tails: vec![],
            max_level: max,
            length: 0
        }
    }




    pub fn get_level(&self) -> usize {
        let mut n = 0;
        while rand::random::<bool>() && n < self.max_level {
            n += 1;
        }
        n
    }
}

