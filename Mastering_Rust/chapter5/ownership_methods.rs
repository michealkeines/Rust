struct Item(u32);

impl Item {
    fn new() -> Self {
        Item(200)
    }

    fn take_item(&self) {}
}

fn main() {
    let it = Item::new();
    it.take_item();
    println!("{}", it.0);
}