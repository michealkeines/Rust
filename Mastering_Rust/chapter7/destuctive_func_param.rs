struct Container {
    items_count: u32
}

fn increment_item(Container{ref mut items_count}: &mut Container) {
    
    *items_count += 1;
}

fn calculate_cost(Container { items_count }: &Container) -> u32 {
    let rate = 67;
    rate * items_count
}

fn main() {
    let mut container = Container {
        items_count: 10
    };

    increment_item(&mut container);
    println!("{}",container.items_count);
    let total_cost = calculate_cost(&container);
    println!("Total cost: {}", total_cost);
}

