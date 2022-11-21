fn main() {
    //Define objects in order
    let objects_given = [
        "A Partridge in a Pear Tree",
        "Two Turtle Doves",
        "Three French Hens",
        "Four Calling Birds",
        "Five Gold Rings",
        "Six Geese A-Laying",
        "Seven Swans A-Swimming",
        "Eight Maids A-Milking",
        "Nine Ladies Dancing",
        "Ten Lords A-Leaping",
        "Eleven Pipers Piping",
        "Twelve Drummers Drumming",
    ];

    //Go through items in order
    let mut suffix = "";
    for (i, object) in objects_given.iter().enumerate() {
        //Figure out what suffix to put at the end of the number
        match i {
            0 => suffix = "st",
            1 => suffix = "nd",
            2 => suffix = "rd",
            _ => suffix = "th",
        }

        //Print out the default line with suffix and then objects in order
        println!(
            "On the {}{} day of Christmas my true love gave to me,",
            i + 1,
            suffix
        );

        if i == 0 {
            println!("{}.", object);
        } else {
            for x in (0..(i + 1)).rev() {
                if let 0 = x {
                    println!("And {}.", objects_given[0]);
                } else {
                    println!("{},", objects_given[x]);
                }
            }
        }
    }
}
