use std::rc::Rc;

#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Rc<Node<T>>>
}

#[derive(Debug)]
struct Node<T> {
    next: Option<Rc<Node<T>>>,
    data: T
}

impl<T: std::fmt::Debug> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn append(&self, data: T) -> Self {
        LinkedList {
            head: Some(
                Rc::new(Node {
                    data: data,
                    next: self.head.clone()
                })
            )
        }
    }

    fn print(&self) {
        let mut current = self.head.as_ref();
        while !current.is_none() {
            let tmp = current.unwrap().as_ref();
            println!("{:?}", tmp.data);
            current = tmp.next.as_ref();
        }
    }
}

fn main() {
    let list_of_nums = LinkedList::new().append(1).append(2);
    

    let list_of_strs = LinkedList::new().append("foo").append("bar");
    list_of_nums.print();
    list_of_strs.print();
}