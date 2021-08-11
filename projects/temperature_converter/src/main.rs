use std::io;

fn convert_Fh(val: i32) -> i32 {
    (val - 32) * 5/9
}

fn convert_Ce(val: i32) -> i32 {
    (val + 9/5) + 32
}

fn read_input(out: &String) -> i32 {
    let mut temp = String::new();
    println!("{}", out);
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to readline");
    let input: i32 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Enter a valid Number");
            32
        }
    };
    input
}

fn main() {
    println!("Temperature Converter");
    loop {
        println!("Options");
        println!("1. Fahrenheit to Celsius");
        println!("2. Celsius to Fahrenheit");
        println!("3. Exit");
        let option_string: String = "Enter you number: ".to_string();
        let option = read_input(&option_string);

        if option == 1 {
            let option_1: String = "Enter the fh".to_string();
            let t: i32 = read_input(&option_1);
            let r: i32 = convert_Fh(t);
            println!("Converted value: {}", r);
        } else if option == 2 {
            let option_2: String = "Enter the ce".to_string();
            let t: i32 = read_input(&option_2);
            let r: i32 = convert_Ce(t);
            println!("Converted value: {}", r); 
        } else {
            println!("Exiting!");
            break;
        }
        
    }
}
