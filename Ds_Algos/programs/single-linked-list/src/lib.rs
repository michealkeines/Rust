use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
#[derive(Debug)]
pub struct Node {
    value: String,
    next: Option<Rc<RefCell<Node>>>
}


impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value: value,
            next: None
        }))
    }
}

pub struct TransactionLog {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
    pub length: u64
}

impl TransactionLog {
    fn new() -> TransactionLog {
        TransactionLog {
            head: None,
            tail: None,
            length: 0
        }
    }

    pub fn push(&mut self, value: String) {
        let new = Node::new(value);
        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(new.clone()), 
            None => self.head = Some(new.clone())
        }; 
        self.length += 1;
        self.tail = Some(new);
    }

    // fn print(&mut self) {
    //     if let Some(n1) = self.head.clone() {
    //         println!("{}", n1.borrow().value);
    //         if let Some(n2) = n1.borrow().next.clone() {
    //             println!("{}", n1.borrow().value);
    //         }
    //     }
        
    // }

    // fn insert(&mut self, value: String, index: u64) {
    //     if index == 1 && self.length == 1 {
    //         let node = Node::new(value.clone());

    //         if let Some(second) = self.head.take() {
    //                 node.borrow_mut().next = Some(second.clone());
    //                 self.head = Some(node.clone());
    //         }
    //     } else {
    //         let mut prev: Option<Rc::<RefCell<Node>>> = None;
    //         let mut cur: Option<Rc::<RefCell<Node>>> = None;

    //         for i in 1..index {
    //             if i == 1 {
    //                 if let Some(val) = self.head.take() {
    //                     prev = Some(val.clone());
    //                     if let Some(c) = &val.borrow_mut().next {
    //                         cur = Some(c.clone());
    //                     }
    //                 }
    //             } else {
    //                 prev = cur.clone();
    //                 if let Some(c) = cur {
    //                     cur = c.borrow_mut().next.clone();
    //                 }
    //             }
    //         }
    //         let node = Node::new(value);
    //         if let Some(n) = prev {
    //             n.borrow_mut().next = Some(node.clone());
    //         }
    //         node.borrow_mut().next = cur.clone();
    //     }
    //     self.length += 1;
    // }

    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.length -= 1;
            Rc::try_unwrap(head)
                .ok()
                .expect("Something is terribly wrong")
                .into_inner()
                .value.clone()
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_nodes() {
        let a = Node::new("test".to_string());
        assert_eq!(a.borrow().value, String::from("test"));
    }

    #[test]
    fn test_log_nodes() {
        
        let mut log = TransactionLog::new();
        log.push("test1".to_string());
        log.push("test2".to_string());
        log.push("test3".to_string());
        log.push("test4".to_string());
     //   log.insert("test5".to_string(), 2);
      
        for i in 0..4 {
            if let Some(a) = log.pop() {
                println!("Popped value: {}",a);           
            }
        }
 
    }
}