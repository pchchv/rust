use std::io;

fn get_celsius(ftemp: f64) -> f64 {
    let ctemp = f64::from(ftemp - 32.0) * (5.0 / 9.0);

    return ctemp;
}

fn main() {
    println!("Fahrenheit to Celsius temperature converter");
    println!("To exit, type 'exit'");

    loop {
        println!("Enter Fahrenheit temperature:");

        let mut ftemp = String::new();

        io::stdin()
            .read_line(&mut ftemp)
            .expect("Failed to read line");

        let ftemp = ftemp.trim();

        if ftemp == "exit" {
            break;
        }

        let ftemp: f64 = match ftemp.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let ctemp = get_celsius(ftemp);

        println!(" Celsius temperature: {ctemp}")
    }
}
