fn main() {
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth",
    ];

    let mut i = 0;
    while i < 12 {
        println!(
            "\nOn the {} day of Christmas, my true love sent to me:",
            days[i]
        );

        let mut j = i;
        while j > 0 {
            println!("{}", gifts[j]);
            j -= 1;
        }

        if i > 0 {
            println!("And {}", gifts[0].to_lowercase());
        } else {
            println!("{}", gifts[0]);
        }

        i += 1;
    }
}