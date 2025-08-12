// Write a program that prints the lyrics to the song "The Twelve Days of Christmas."

fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let gifts = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five golden rings",
        "six geese a laying",
        "seven swans a swimming",
        "eight maids a milking",
        "nine ladies dancing",
        "ten lords a leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    for (index, day) in days.iter().enumerate() {
        println!("On the {day} day of Christmas my true love gave to me");
        for i in (0..index + 1).rev() {
            if i == 0 && index > 0 {
                println!("and {}", gifts[i]);
            } else {
                println!("{}", gifts[i]);
            }
        }
        println!("");
    }
}
