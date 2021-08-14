rust doenst have exceptions

instead it has two type of errors

recoverable errors with type Result<T, E>

unrecoverable errors with type panic

we use panic! macro to make the program panic at any place

Result<> has two things, Ok to get the value and Err to get the error message

match test {
	Ok(num) => num,
	Err(_) => println("not a number")
}

`
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let temp = File::open("ok.txt");

    let data = match temp {
        Ok(val) => val,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("ok.txt") {
                Ok(t) => t,
                Err(e) => panic!("privilege not enough!")
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        }
    };
}

`

there are method like unwrap _or_else, unwrap, expect that are shortcuts instead of using match to mkae the code clean


