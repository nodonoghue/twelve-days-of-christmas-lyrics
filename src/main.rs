fn main() {
    let day_lyrics = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three french hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let day_number = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    for number in 0..12 {
        println!(
            "On the {} day of Christmas my true love gave to me",
            day_number[number]
        );
        for day_number in (0..number + 1).rev() {
            println!("{}", day_lyrics[day_number]);
        }
        println!();
    }
}
