fn main() {
    let lyrics = [
        "a Partridge in a Pear Tree",
        "Two Turtle Doves",
        "Three French Hens",
        "Four Calling Birds",
        "Five Gold Rings",
        "Six Geese a Laying",
        "Seven Swans a Swimming",
        "Eight Maids a Milking",
        "Nine Ladies Dancing",
        "Ten Lords a Leaping",
        "Eleven Pipers Piping",
        "Twelve Drummers Drumming",
    ];

    for day in 0..12 {
        println!(
            "On the {} day of Christmas, my true love gave to me",
            days_to_ordinal(day + 1)
        );
        for lyric in (0..=day).rev() {
            if day > 0 && lyric == 0 {
                print!("and ");
            }
            println!("{}", lyrics[lyric]);
        }
        println!();
    }
}

fn days_to_ordinal(day: usize) -> String {
    match day {
        1 => "first".to_string(),
        2 => "second".to_string(),
        3 => "third".to_string(),
        4 => "fourth".to_string(),
        5 => "fifth".to_string(),
        6 => "sixth".to_string(),
        7 => "seventh".to_string(),
        8 => "eighth".to_string(),
        9 => "ninth".to_string(),
        10 => "tenth".to_string(),
        11 => "eleventh".to_string(),
        12 => "twelfth".to_string(),
        _ => format!("{}th", day),
    }
}
