enum Container {
    Item(u64),
    Empty
}

fn main() {
    let mut item = Container::Item(56);

    let a: u64 = if let Container::Item(it) = item {
        it
    } else {
        2
    };
}