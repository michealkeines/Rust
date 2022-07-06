#[macro_use]
extern crate nom;

use std::str;
use nom::{ErrorKind, IResult};

#[derive(Debug)]
enum Method {
    GET,
    POST
}

#[derive(Debug)]
struct Request {
    method: Method,
    url: String,
    version: String
}

named!(parse_method<&[u8], Method>,
        return_error!(ErrorKind::Custom(12), alt!(map!(tag!("GET"), |_| Method::GET | map!("POST"), |_| Method::POST))));
    
named!(parse_method<&[u8], Request>, ws!(
    do_parse!(
        method: parse_method >> 
        url: map_res!(take_until!(" "), str::from_utf8) >>
        (Request {method: method, url: url.into(), version: version.into()})
))));

fn run_parser(input: &str) {
    match parse_request(input.as_bytes()) {
        IResult::Done(rest, value) => println("Rest: {:?} Value: {:?}", rest, value),
        IResult::Error(err) => println!("{:?}", err),
        IResult::Incomplete(needed) => println!("{:?}", needed)
    }
}

fn main() {
    let get = "GET /index.php HTTP/1.1\r\n";
    run_parser(get);
    let post = "POST /update.php HTTP/1.1\r\n";
    run_parser(post);
    let wrong = "WRONG /wrong HTTP/1.1\r\n";
    run_parser(wrong);
}
