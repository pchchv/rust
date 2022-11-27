use std::io;

fn pigify(word: &str) -> String {
    let mut letters = word.chars();
    let first_letter = letters.next().unwrap();

    let lowercase_first_letter = first_letter.to_lowercase().next().unwrap();

    match lowercase_first_letter {
        'a' | 'e' | 'i' | 'o' | 'u' => format!("{}-hay", word),
        _ => format!("{}-{}ay", letters.as_str(), lowercase_first_letter),
    }
}

fn main() {
    let mut input = String::new();
    let mut result = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.split(" ");

    for word in input {
        let w = pigify(word);
        result.push_str(&w);
        result.push_str(" ")
    }

    println!("{}", result.trim());
}
