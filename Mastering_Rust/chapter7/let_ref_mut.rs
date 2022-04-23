#[derive(Debug)]
struct Items(u32);

fn main() {
    let item = Items(2);
    let items_ptr = &item;
    let ref items_ref = item;

    assert_eq!(items_ptr as *const Items, items_ptr as *const Items);

    let mut a = Items(20);

    {
        let ref mut b = a;
        b.0 += 25;
    }

    println!("{:?}", item);
    println!("{:?}", a);

}

