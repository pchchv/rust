use std::io;

fn main() {
    let mut v: Vec<i64> = Vec::new();

    println!("Input 'q' or 'quit' to stop");

    loop {
        let mut input = String::new();

        println!("Input a value to add to the integer list: ");

        io::stdin().read_line(&mut input).expect("Input invalid!");

        let input = input.trim();

        if input == "q" || input == "quit" {
            break;
        }

        let input: i64 = match input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input invalid, try again!");
                continue;
            }
        };

        v.push(input);
    }
}
