fn get_nth(items: &Vec<usize>, nth: usize) -> Option<usize> {
    if nth < items.len() {
        Some(items[nth])
    } else {
        None
    }
}

fn double(val: usize) -> usize {
    val * val
}

fn main() {
    let items = vec![7, 6, 4, 3, 5, 3, 10, 3, 2, 4];
    println!("{}",items.len());
    //let doubled = get_nth(&items, 4).map(double);
    let doubled = get_nth(&items, 3).map(|v| v * 2);
    println!("{:?}", doubled);
}