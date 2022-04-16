enum Level {
    Error
}

struct Logger<'a>(&'a str, Level);

fn configure_logger<'a, T>(t: T) where T: Send + 'a{

}

fn main() {
    let other = String::from("test");

    let log2 = Logger(&other, Level::Error);
    configure_logger(log2);
}