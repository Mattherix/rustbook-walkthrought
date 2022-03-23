fn main() {
    // Conversion
    println!("0 celsius = {} fahrenheit", to_fahrenheit(0 as f32));
    println!("0 fahrenheit = {} celsius\n", to_celsius(0 as f32));

    // Fibonacci
    for n in 0..17 {
        println!("fibonacci({n}) = {}", fibonacci(n));
    }
    println!();

    // The Twelve Days of Christmas (Christmas carol)
    carol();
}

fn to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32 as f32) * (5 as f32 / 9 as f32)
}

fn to_fahrenheit(celsius: f32) -> f32 {
    celsius * (9 as f32 / 5 as f32) + 32 as f32
}

fn fibonacci(nth: u32) -> u32 {
    if nth < 2 {
        nth
    } else {
        fibonacci(nth - 1) + fibonacci(nth - 2)
    }
}

fn carol() {
    println!(" -- The Twelve Days of Christmas -- \n");
    // from: https://genius.com/Christmas-songs-the-twelve-days-of-christmas-lyrics

    let ordinals = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "night",
        "tenth", "eleventh", "twelfth",
    ];
    let phrases = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three french hens",
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

    for (i, ordinal) in ordinals.iter().enumerate() {
        println!(
            "On the {} day of Christmas, my true love sent to me",
            ordinal
        );
        for phrase in phrases[0..i+1].iter().rev() {
            println!("{}", phrase);
        }
        println!();
    }
}
