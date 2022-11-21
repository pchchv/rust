use std::io;

fn fibonacci(n: u128) -> u128 {
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}
fn main() {
    loop {
        println!("Enter the number of the desired number of the Fibonacci sequence");
        println!("To exit, type 'exit'");

        let mut num = String::new();

        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line");

        let num = num.trim();

        if num == "exit" {
            break;
        }

        let num: u128 = match num.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let num = fibonacci(num);
        println!("{num}")
    }
}
