use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct GroundStation {
    radio_freq: f64
}

fn main() {
    let base: Rc<RefCell<GroundStation>> = Rc::new(RefCell::new(
        GroundStation {
            radio_freq: 87.34
        }
    ));

    println!("{:?}", base);

    {
        let mut b = base.borrow_mut();
        b.radio_freq -= 12.1;
        println!("{:?}", base);
        println!("b: {:?}", b);
    }

    println!("{:?}", base);

    let mut b = base.borrow_mut();
    b.radio_freq += 2.0;

    println!("base {:?}", base);
    println!("{:?}", b);
}