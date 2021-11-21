use std::mem;

type Link<T> = Option<Box<Node<T>>>;


pub struct Node<T> {
    value: T,
    next: Link<T>
}

pub struct List<T> {
    head: Link<T>
}

struct Iterlist<T>(List<T>);

impl<T> Iterator for Iterlist<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

struct Iter<'a, T> {
    next: Option<&'a Node<T>>
}

impl<T> List<T> {
    fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            next: self.head.as_deref()
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|n|{
            self.next = n.next.as_deref();
            &n.value
        })
    }
}

impl<T> List<T> {
    fn into_iter(self) -> Iterlist<T> {
        Iterlist(self)
    }
}

struct Itermut<'a, T> {
    next: Option<&'a mut Node<T>>
}

impl<T> List<T> {
    fn iter_mut<'a>(&'a mut self) -> Itermut<'a, T> {
        Itermut{
            next: self.head.as_deref_mut()
        }
    }
}

impl<'a, T> Iterator for Itermut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node|{
            self.next = node.next.as_deref_mut();
            &mut node.value
        })
    }
}

impl<T> List<T> {
    fn new() -> Self {
        List { head: None }
    } 
}

impl<T> List<T> {
    fn push(&mut self, value: T) {
        let new_node = Box::new(
            Node {
                value: value,
                next: mem::replace(&mut self.head, None)
            }
        );
        mem::replace(&mut self.head, Some(new_node));
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            let val = node.value;
            mem::replace(&mut self.head,node.next);
            val
        })
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.value
        })
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.value
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut current_node = mem::replace(&mut self.head, None);

        while let Some(mut temp_node) = current_node {
            current_node = mem::replace(&mut temp_node.next, None);
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push("1");
        list.push("2");
        list.push("3");

        assert_eq!(list.pop(), Some("3"));
        assert_eq!(list.pop(), Some("2"));

        list.push("4");
        list.push("5");

        assert_eq!(list.pop(), Some("5"));
        assert_eq!(list.pop(), Some("4"));

        assert_eq!(list.pop(), Some("1"));
        assert_eq!(list.pop(), None);
    }
    #[test]
    fn peek() {
        let mut list = List::new();

        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(),None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        list.peek_mut().map(|value|{
            *value = 4;
        });

        assert_eq!(list.peek(), Some(&4));
        assert_eq!(list.pop(), Some(4));
    }
    #[test]
    fn into_iter() {
        let mut list = List::new();

        list.push(0);
        list.push(1);
        list.push(2);
        list.push(3);
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(0);
        list.push(1);
        list.push(2);
        list.push(3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), None);
        assert_eq!(list.peek(), Some(&3));
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(0);
        list.push(1);
        list.push(2);
        list.push(3);
        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 0));
        assert_eq!(iter.next(), None);
        assert_eq!(list.peek(), Some(&3));
    }
}