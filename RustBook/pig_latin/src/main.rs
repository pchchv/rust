use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.split(" ");

    for i in input {
        println!("{i}");
    }
}
