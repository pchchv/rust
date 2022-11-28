use std::collections::HashMap;
use std::io;

fn adder<'a>(dept: &str, mut name: Vec<&'a str>, staff: &mut HashMap<String, Vec<&'a str>>) {
    // Need fix mutable
    let dept_staff = staff.get(dept);
    let mut v: Vec<&str> = Vec::new();

    if dept_staff != Option::None {
        let mut dept_staff = dept_staff.unwrap().clone();
        v.append(&mut dept_staff)
    }

    v.append(&mut name);

    staff.insert(dept.to_string(), name);
}

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
    println!("To add an employee to the department, type 'Add <Name> to <Department>'");
    println!("To view the employees of a department, enter the name of department");
    println!("To exit program, type <Exit>");

    loop {
        let mut staff: HashMap<String, Vec<&str>> = HashMap::new();
        let mut command: Vec<&str> = Vec::new();
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = String::from(input.trim());

        for word in input.split_whitespace() {
            command.push(word);
        }

        let first_word = command[0].to_string();

        if first_word == "Add" {
            adder(command[3], vec![command[1]], &mut staff);
        } else if first_word == "Exit" {
            break;
        } else {
            getter(command[0], &staff);
        }
    }
}
