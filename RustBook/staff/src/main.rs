use std::collections::HashMap;
use std::io;

fn main() {
    let mut staff: HashMap<String, Vec<&str>> = HashMap::new();

    println!("To add an employee to the department, type 'Add <Name> to <Department>'");
    println!("To view the employees of a department, enter the name of department");
    println!("To exit program, type <Exit>");

    loop {
        let mut command: Vec<&str> = Vec::new();
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = String::from(input.trim());
        let input = input.split_whitespace();

        for word in input {
            command.push(word.trim());
        }

        let first_word = command[0].trim().to_string();

        if first_word == "Add" {
            let department = staff.entry(command[3].to_string()).or_insert(vec![]);

            department.push(command[1]);
        } else if first_word == "Exit" {
            break;
        } else {
            let values = staff.get_mut(&first_word).expect("No such department");
            values.sort();
            println!("{}:", first_word);
            for value in values {
                println!("\t{}", value);
            }
        }
    }
}
