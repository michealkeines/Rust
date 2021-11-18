use std::rc::Rc;
use std::mem;

type Link<T> = Option<Rc<Node<T>>>;

struct List<T> {
    head: Link<T>
}

struct Node<T> {
    value: T,
    next: Link<T>
}

struct Iter<'a, T> {
    next: Option<&'a Node<T>>
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.value
        })
    }
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref()
        }
    }
}

impl<T> List<T> {
    pub fn new() -> List<T> {
       List{ 
           head: None
        }
    }
}

impl<T> List<T> {
    pub fn prepend(&self, value: T) -> List<T> {
        List {
            head: Some(
                Rc::new(
                    Node {
                        value: value,
                        next: self.head.clone()
                    }
                )
            )
        }
    }

    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|node| {
                node.next.clone()
            })
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.value
        })
    }
}

#[cfg(test)]
mod  test {
    use super::*;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(),None);

        let list = list.prepend(1).prepend(2).prepend(3);

        assert_eq!(list.head(),Some(&3));

        let list = list.tail();

        assert_eq!(list.head(),Some(&2));
        let list = list.tail();
        assert_eq!(list.head(),Some(&1));
        let list = list.tail();
        assert_eq!(list.head(), None);

        let list = list.tail();
        assert_eq!(list.head(),None);

    }

    fn iter() {
        let list = List::new().prepend(1).prepend(2).prepend(3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}
