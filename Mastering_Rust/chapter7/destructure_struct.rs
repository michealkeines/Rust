enum Food {
    Pizza,
    Salad
}

enum PaymenMode {
    Bitcoin,
    Credit
}

struct Order {
    count: u8,
    item: Food,
    payment: PaymenMode
}

struct test {
    tet: u32
}

fn main() {
    let food_order = Order {
        count: 2,
        item: Food::Salad,
        payment: PaymenMode::Credit
    };

    let te = test {
        tet: 34
    };

    let a = if let Order{ count, .. } = te {
        println!("{:?}", count);
        1
    } else {
        2
    }


}