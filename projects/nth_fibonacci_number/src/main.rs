use std::io;

fn main() {
    println!("-----------nTh Fibonacci number-----------");
    println!("Enter the value for n: ");
    
    let mut val = String::new();

    io::stdin()
        .read_line(&mut val)
        .expect("Invalid Entry");
        
    let n: i32 = match val.trim().parse() {
                    Ok(num) => num,
                    Err(_) => 0
                };

    println!("Nth Fibonacci number: {}",fib(n));
    

}

fn fib(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    return fib(n-1) + fib(n-2);
}


