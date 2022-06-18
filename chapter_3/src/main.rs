use std::num::ParseIntError;

pub const CELSIUS_TO_FAHRENHEIT: f64 = 1.8;
pub const FAHRENHEIT_TO_CELSIUS: f64 = 5.0 / 9.0;
pub const CHRISTMAS_LYRICS: [(&str, &str); 12] = [
    ("first", "A patridge in a pear tree"),
    ("second", "Two turtle doves,and"),
    ("third", "Three french hens"),
    ("fourth", "Four calling birds"),
    ("fifth", "Five golden rings"),
    ("sixth", "Six geese a-laying"),
    ("seventh", "Seven swans a-swimming"),
    ("eight", "Eight maids a-milking"),
    ("ninth", "Nine ladies dancing"),
    ("tenth", "Ten lords a-leaping"),
    ("eleventh", "Eleven pipers piping"),
    ("twelfth", "Twelve drummers drumming"),
];

fn main() {
    println!("Hello, world!");

    loop {
        print_choose_text();
        println!("Input");
        let user_input = match get_user_numerical_input() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number");
                continue;
            }
        };

        match user_input {
            1 => {
                print_choose_conversion();
                let conversion_direction = match get_user_numerical_input::<u32>() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Not a number");
                        continue;
                    }
                };
                handle_conversion(conversion_direction);
            }
            2 => {
                print_choose_number_of_verses();
                match get_user_numerical_input::<u32>() {
                    Ok(num) => match num.try_into() {
                        Ok(size) => match size {
                            1..=12 => handle_christmas(size),
                            _ => {
                                println!(
                                    "number of verses must be between 1 and 12. Value was {}",
                                    size
                                );
                            }
                        },
                        Err(_) => {
                            println!("Unable to convert");
                            continue;
                        }
                    },
                    Err(err) => {
                        println!("unable to convert: {:?}", err);
                        continue;
                    }
                };
            }
            0 => break,
            _ => println!("Not supported"),
        };
    }
}

fn handle_christmas(number_of_verses: usize) {
    for verse in 0..number_of_verses {
        let first_line = handle_first_verse(verse);
        println!("[Verse {}]", verse + 1);
        println!("{}", first_line);
        handle_other_verses(verse);
        println!();
    }
    println!();
}

fn handle_other_verses(verse: usize) {
    for i in (0..=verse).rev() {
        let (_day, lyric) = CHRISTMAS_LYRICS[i];
        println!("{}", lyric);
    }
}

fn handle_first_verse(verse: usize) -> String {
    let (day, _) = CHRISTMAS_LYRICS[verse];
    return format!("On the {} day of Christmas, my true love sent to me", day);
}

fn print_choose_number_of_verses() {
    println!("Number of verses can be between 1 and 12, including both numbers");
    println!("Number of verses:");
}

fn print_choose_text() {
    println!("Please choose:");
    println!("1: Fahrenheit/Celsius Conversion");
    println!("2: Print christmas song lyrics");
    println!("0: Quit");
}

fn print_choose_conversion() {
    println!("Please choose:");
    println!("1: Fahrenheit -> Celsius");
    println!("2: Celsius -> Fahrenheit");
    println!();
}

fn get_user_numerical_input<T: std::str::FromStr<Err = ParseIntError>>() -> Result<T, ParseIntError>
{
    let mut user_input = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("failed to read line");

    user_input.trim().parse::<T>()
}

fn handle_conversion(conversion_direction: u32) {
    match conversion_direction {
        1 => {
            println!("Fahrenheit value please");
            let fahrenheit: f64 = match get_user_numerical_input::<i32>() {
                Ok(num) => num.into(),
                Err(_) => {
                    println!("Not a number");
                    return;
                }
            };
            let celsius = convert_fahrenheit_to_celsius(fahrenheit);
            println!("Fahrenheit -> Celsius == {} -> {}", fahrenheit, celsius);
            println!();
        }
        2 => {
            println!("celsius value please");
            let celsius: f64 = match get_user_numerical_input::<i32>() {
                Ok(num) => num.into(),
                Err(_) => {
                    println!("Not a number");
                    return;
                }
            };
            let fahrenheit = convert_celsius_to_fahrenheit(celsius);
            println!("Celsius -> Fahrenheit == {} -> {}", celsius, fahrenheit);
            println!();
        }
        _ => println!("Unknown/not supported"),
    }
}

fn convert_fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    FAHRENHEIT_TO_CELSIUS * (fahrenheit - 32.0)
}

fn convert_celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * CELSIUS_TO_FAHRENHEIT) + 32.0
}
