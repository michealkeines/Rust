use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct Node {
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
    value: String
}

#[derive(Clone)]
pub struct TransactionLog {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
    pub lenght: u64
}

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>>{
        Rc::new(RefCell::new(Node { prev: None, next: None, value: value }))
    }
}

pub struct ListIterator {
    current: Option<Rc<RefCell<Node>>>
}

impl ListIterator {
    fn new(start_at: Option<Rc<RefCell<Node>>>) -> ListIterator {
        ListIterator {
            current: start_at
        }
    }
}

impl Iterator for ListIterator {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        let current = &self.current;
        let mut result = None;

        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.value.clone());
                current.next.clone()
            },
            None => None
        };
        result
    }
}

impl DoubleEndedIterator for ListIterator {
    fn next_back(&mut self) -> Option<String> {
        let current = &self.current;
        let mut result = None;
        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.value.clone());
                current.prev.clone()
            },
            None => None
        };
        result
    }
}

impl TransactionLog {
    fn new() -> TransactionLog {
        TransactionLog {
            head: None,
            tail: None,
            lenght: 0
        }
    }

    fn push(&mut self,value: String) {
        let node = Node::new(value);

        match self.tail.take() {
            Some(n) =>{node.borrow_mut().prev =Some(n.clone()); n.borrow_mut().next = Some(node.clone())},
            None => self.head = Some(node.clone())
        };
        self.lenght += 1;

        self.tail = Some(node);
    }

    fn pop(&mut self) -> Option<String> {
        self.head.take().map(
            |head| {
                if let Some(n) = head.borrow_mut().next.take() {
                    n.borrow_mut().prev = None;
                    self.head = Some(n);
                } else {
                    self.tail.take();
                }
                self.lenght -= 1;
                Rc::try_unwrap(head)
                        .ok()
                        .expect("sdsfd")
                        .into_inner()
                        .value
            })
    }

 // }
        

}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test() {
    //     let mut log = TransactionLog::new();
    //     log.push(String::from("test1"));
    //     log.push(String::from("test2"));
    //     log.push(String::from("test3"));


    //     for i in 0..3 {
    //         println!("{:?}",log.pop());
    //     }
    // }
    #[test]
    fn test_iter() {
        let mut log = TransactionLog::new();
        log.push(String::from("test1"));
        log.push(String::from("test2"));
        log.push(String::from("test3"));

        let list = ListIterator::new(log.head);

        for i in list {
            println!("{}",i);
        }
    }
}