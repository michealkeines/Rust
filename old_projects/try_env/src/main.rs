use std::io;

fn main() {
    const x: u32 = 5;

    println!("The value of x is: {}", x);
    let t = x + 6;
    println!("The value of x is: {}", x);
    let t = x + 2;

    let mut spaces = "  ";
    println!("{}", spaces);
    let spaces = spaces.len();
    println!("{}", spaces);

    //let guess = "42".parse().expect("Not a number");

    let guess: u32 = "42".parse().expect("Not a number");

    let test: bool = true;

    let a = [1,2,3,4,4,5];

    let mut index = String::new();

    println!("Enter the index: ");

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let test = a[index];

    println!("index: {}, value: {}", index, test);

    testing();
    
    println!("what function {}",what(10));

}

fn testing() {
    println!("testing");
}

fn what(x: i32) -> i32 {
    let mut c = 0;
    let y = loop {
        c += 1;
        if c == 10 {
            break c * 2;
        }
    };
    return y;
}
