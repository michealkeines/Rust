use std::io;


fn main() {
    let mut test = 1;

    while test != 3 {
        println!("--------------Temperatur Converter--------------");
        println!("1.Fahrenheit to Celsius");
        println!("2.Celsius to Fahrenheit");
        println!("3.Exit");

        let mut val = String::new();

        println!("Enter your option: ");
        io::stdin()
            .read_line(&mut val)
            .expect("Invalid");

        let option: i32 = match val.trim().parse() {
            Ok(num) => num,
            Err(_) => continue 
        };
        //println!("Your option: {}", option);
        if option == 1 {
            fah_to_cel();
        } else if option == 2 {
            cel_to_fah();
        } else if option == 3 {
            test = 3;
        } else {
            println!("Invalid Option.");
        }
    }
}

fn get_input_temperature() -> f64 {
    let mut val = String::new();
    println!("Enter the temperature: ");
    io::stdin()
        .read_line(&mut val)
        .expect("invalid Temperature");

    let temperature: f64 = match val.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            get_input_temperature()
        }
    };

    return temperature;
}

fn fah_to_cel() {
    let temperature = get_input_temperature();

    let result = (temperature - 32.0) / 1.8;

    println!("Converted value: {}", result);
}

fn cel_to_fah() {
    let temperature = get_input_temperature();

    let result = (temperature * 1.8) + 32.0;

    println!("Converted value: {}", result);
}
