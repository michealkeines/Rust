enum Level {
    Error
}

struct Logger<'a>(&'a str, Level);

fn configure_logger<T>(t: T) where T: Send + 'static {

}

fn main() {
    let name = "global";
    let log1 = Logger(name, Level::Error);
    configure_logger(log1);
}