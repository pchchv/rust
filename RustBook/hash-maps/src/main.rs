fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // // let team_name = String::from("Blue");
    // // let score = scores.get(&team_name).copied().unwrap_or(0);

    // for (key, value) in &scores {
    //     println!("{key}: {value}");
    // }

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Blue"), 25);

    // println!("{:?}", scores);

    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}
