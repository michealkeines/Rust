use std::io;

fn main() {
    println!("Enter a number: ");
    let mut val: u32 = 0;
    loop {
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Unable to readline");
        let test: (u32, bool) = match input.trim().parse() {
            Ok(num) => (num,true),
            Err(_) => {
                println!("Try again with a valid number");
                (0,false)
            }
        };
        if test.1 == true {
            val = test.0;
            break;
        } else {
            continue;
        }
    }
    let res: u32 = recur(val);
    println!("Result: {} ", res);
}

fn recur(val: u32) -> u32 {
    if val <= 1{
        return val;
    } else {
        return recur(val - 1) + recur(val - 2);
    }
}
