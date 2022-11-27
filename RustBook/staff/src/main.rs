use std::collections::HashMap;
use std::io;

fn adder(command: Vec<&str>, staff: &HashMap<String, Vec<&str>>) {}

fn getter(dept: &str, staff: &HashMap<String, Vec<&str>>) {
    let dept_staff = staff.get(dept);

    if dept_staff == Option::None {
        println!("Department not found!");
        println!("Maybe the employees of this department have not yet been added");
    } else {
        println!("Staff of {dept} department: ");

        for name in dept_staff.unwrap().iter() {
            println!("    {name}");
        }
    }
}

fn main() {
    let mut input = String::new();
    let mut command: Vec<&str> = Vec::new();
    let staff: HashMap<String, Vec<&str>> = HashMap::new();

    println!("To add an employee to the department, type 'Add <Name> to <Department>'");
    println!("To view the employees of a department, enter the name of department");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut i = 0;

    for word in input.split(" ") {
        if i == 0 && (word != "Add" || word != "add") {
            getter(word, &staff);
            break;
        } else if i == 1 || i == 3 {
            command.push(word)
        }
        i += 1;
    }

    if i == 3 {
        adder(command, &staff)
    }
}
