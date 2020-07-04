use std::env;
use std::str::FromStr;
use std;

fn main() {

    let args: Vec<String> = env::args().collect();
    let x: i64 = i64::from_str(args.get(1).unwrap()).unwrap();

    if env::args().count() != 2 {
        println!("Please enter a single integer as a command line argument.");
    }

    if x < 0 {
        println!("minus {}", num_as_words(-1 * x));
    } else {
        println!("{}", num_as_words(x));
    }
}

fn num_as_words(num: i64) -> String {
    let mut result: String = digit_as_word(num % 10);
    let mut next_digits: i64 = get_next_digits(num);
    let mut more_digits: bool = next_digits > 0;
    while more_digits {
        result = format!("{} {}", digit_as_word(next_digits % 10), result);
        next_digits = get_next_digits(next_digits);
        more_digits = next_digits > 0;
    }
    result
}

fn get_next_digits(num: i64) -> i64 {
    (num - num % 10) / 10
}

fn digit_as_word(digit: i64) -> String {
    match digit {
        0 => "zero".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        _ => "error".to_string()
    }
}