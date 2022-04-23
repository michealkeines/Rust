struct Person(String);

fn main() {
    let mut a = Person("Richard Feynman".to_string());
    match a {
        Person(ref mut name) => {
            name.push_str(" who");
            println!("{} was a great Physicist!", name)},
        _ => panic!("Oh no!")
    }

    let b = a;
}