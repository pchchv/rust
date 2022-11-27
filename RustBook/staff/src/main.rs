use std::collections::HashMap;
use std::io;

fn adder(command: String, staff: &HashMap<String, Vec<&str>>) {}

fn getter(dept: &str) {}

fn main() {
    let staff: HashMap<String, Vec<&str>> = HashMap::new();
    let mut command = String::new();
    let mut input = String::new();

    println!("To add an employee to the department, type 'Add <Name> to <Department>'");
    println!("To view the employees of a department, enter the name of department");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut i = 0;

    for word in input.split(" ") {
        if i == 0 && (word != "Add" || word != "add") {
            getter(word)
        } else if i == 1 || i == 3 {
            command.push_str(word)
        }
        i += 1;
    }

    if i > 1 {
        adder(command, &staff)
    }
}
