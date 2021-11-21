use std::rc::Rc;
use std::cell::{Ref, RefMut, RefCell};
use std::mem;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

pub struct Node<T> {
    value: T,
    next: Link<T>,
    prev: Link<T>
}

impl<T> Node<T> {
    fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            value: value,
            next: None,
            prev: None
        }))
    }
}

impl<T> List<T> {
    fn new() -> Self {
        List {
            head: None,
            tail: None
        }
    }
}

impl<T> List<T> {
    pub fn push_back(&mut self, value: T) {
        let new_node = Node::new(value);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_node);   
            }, 
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
    }
    pub fn push_front(&mut self, value: T) {
        let new_node = Node::new(value);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_node.clone());
                new_node.borrow_mut().next = Some(old_head);
                self.head = Some(new_node);
            },
            None => {
                self.tail = Some(new_node.clone());
                self.head = Some(new_node);
            }
        }
    }

    pub fn pop_tail(&mut self) -> Option<T> {
        match self.tail.take() {
            Some(popped_tail) => {
                let updated_tail = popped_tail.borrow_mut().prev.take();
                if let Some(node) = updated_tail {
                    node.borrow_mut().next = None;
                    self.tail = Some(node);
                } else {
                    self.head = None;
                    self.tail = None;
                }
                if let Ok(val) = Rc::try_unwrap(popped_tail) {
                    Some(val.into_inner().value)
                } else {
                    None
                }
            },
            None => None
        }
    }
    pub fn pop_front(&mut self) -> Option<T> {
        match self.head.take() {
            Some(popped_head) => {
                let updated_head = popped_head.borrow_mut().next.take();
                if let Some(node) = updated_head {
                    node.borrow_mut().prev = None;
                    self.head = Some(node);
                } else {
                    self.head = None;
                    self.tail = None;
                }
                if let Ok(val) = Rc::try_unwrap(popped_head) {
                    Some( val.into_inner().value)
                } else {
                    None
                }
            },
            None => None
        }
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail.as_ref().map(|node| {
            Ref::map(node.borrow(), |n| {
                &n.value
            })
        })
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head.as_ref().map(|node| {
            Ref::map(
                node.borrow(), |n| {
                    &n.value
                }
            )
        })
    }
    pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        self.tail.as_ref().map(|node| {
            RefMut::map(node.borrow_mut(), |n| {
                &mut n.value
            })
        })
    }

    pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        self.head.as_ref().map(|node| {
            RefMut::map(
                node.borrow_mut(), |n| {
                    &mut n.value
                }
            )
        })
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.0.pop_tail()
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(),Some(3));
        assert_eq!(iter.next_back(),Some(1));
        assert_eq!(iter.next(),Some(2));
        assert_eq!(iter.next_back(),None);
        assert_eq!(iter.next(),None);
    }

    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(list.peek_front().is_none(), true);
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        list.push_front(4);
        list.push_front(5);

        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);

        assert_eq!(list.pop_tail(), None);

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.pop_tail(), Some(3));
        assert_eq!(list.pop_tail(), Some(2));

        list.push_back(4);
        list.push_back(5);

        assert_eq!(list.pop_tail(), Some(5));
        assert_eq!(list.pop_tail(), Some(4));

        assert_eq!(list.pop_tail(), Some(1));
        assert_eq!(list.pop_tail(), None);
    }

    #[test]
    fn peeks() {
        let mut list = List::new();

        assert_eq!(list.peek_front().is_none(),true);
        assert_eq!(list.peek_back().is_none(),true);
        assert_eq!(list.peek_front_mut().is_none(),true);
        assert_eq!(list.peek_back_mut().is_none(),true);

        list.push_front(1);
        list.push_back(2);
        list.push_front(3);

        assert_eq!(*list.peek_front().unwrap(), 3);
        assert_eq!(&mut *list.peek_front_mut().unwrap(), &mut 3);
        assert_eq!(*list.peek_back().unwrap(), 2);
        assert_eq!(&mut *list.peek_back_mut().unwrap(), &mut 2);

    }
}