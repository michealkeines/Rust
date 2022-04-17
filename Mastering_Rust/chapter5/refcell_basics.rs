use std::cell::RefCell;

#[derive(Debug)]
struct Bag {
    item: Box<u32>
}

fn main() {
    let bag = RefCell::new(
        Bag {
            item: Box::new(23)
        }
    );

    let hand1 = &bag;
    let hand2 = &bag;

    *hand1.borrow_mut() = Bag { item: Box::new(24) };
    *hand2.borrow_mut() = Bag { item: Box::new(23) };

    let borrowed = hand1.borrow();
    let mu = hand1.borrow_mut();
    println!("{:?}", borrowed);
}