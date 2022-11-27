use std::io;

fn main() {
    let mut input = String::new();

    println!("To add an employee to the department, type 'Add <Name> to <Department>'");
    println!("To view the employees of a department, enter the name of department");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.split(" ");
}
