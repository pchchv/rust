use std::io;

fn get_celsius(input_temp: f64) -> f64 {
    let result_temp = f64::from(input_temp - 32.0) * (5.0 / 9.0);

    return result_temp;
}

fn get_fahrenheit(input_temp: f64) -> f64 {
    let result_temp = (f64::from(input_temp) * (9.0 / 5.0)) + 32.0;

    return result_temp;
}

fn main() {
    println!("Fahrenheit to Celsius temperature converter");
    println!("To exit, type 'exit'");

    let mut mode = String::new();

    println!("Enter the converter operation mode");
    println!("'fc' for Fahrenheit to Celsius");
    println!("'cf' for Celsius to Fahrenheit");

    io::stdin()
        .read_line(&mut mode)
        .expect("Failed to read line");

    loop {
        println!("Enter temperature:");

        let mut input_temp = String::new();

        io::stdin()
            .read_line(&mut input_temp)
            .expect("Failed to read line");

        let input_temp = input_temp.trim();

        if input_temp == "exit" {
            break;
        }

        let input_temp: f64 = match input_temp.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if mode == "fc" {
            let result_temp = get_celsius(input_temp);
            println!("Celsius temperature: {result_temp}")
        } else if mode == "cf" {
            let result_temp = get_fahrenheit(input_temp);
            println!("Fahrenheit temperature: {result_temp}")
        } else {
            println!("Enter the correct operating mode of the converter!")
        }
    }
}
