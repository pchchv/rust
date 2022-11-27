use std::io;

fn get_mean(v: &[i64]) -> f64 {
    let mut count: f64 = 0.0;
    let mut sum: f64 = 0.0;

    for num in v {
        sum += *num as f64;
        count += 1.0;
    }

    sum / count
}

fn get_median(v: &mut [i64]) -> f64 {
    let middle = v.len() / 2;
    v.sort();

    if v.len() % 2 == 0 {
        ((v[middle] + v[middle - 1]) as f64) / 2.0
    } else {
        v[middle] as f64
    }
}

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

    println!("Mean: {}", get_mean(&v));
    println!("Median: {}", get_median(&mut v));
}
