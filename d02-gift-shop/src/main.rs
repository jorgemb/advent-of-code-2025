use std::collections::HashSet;
use std::str::FromStr;
use std::{env, fs};

// Returns true if the given number is invalid
fn is_invalid(number: i64) -> bool {
    let as_string = number.to_string();
    for i in 1..=(as_string.len() / 2) {
        // Check if the pattern can repeat exactly for the size of the number
        if as_string.len() % i != 0 {
            continue;
        }

        // Divide the string in chunks and convert to set
        let chunks = as_string
            .as_bytes()
            .chunks(i)
            .map(str::from_utf8)
            .collect::<Result<HashSet<&str>, _>>()
            .unwrap();
        if chunks.len() == 1 {
            println!("{} is not a valid number!", as_string);
            return true;
        }
    }

    // Number is valid
    false
}
fn is_invalid_twice(number: i64) -> bool {
    let as_string = number.to_string();
    // Check if the pattern can repeat exactly for the size of the number
    if as_string.len() % 2 != 0 {
        return false;
    }

    // Divide the string in chunks and convert to set
    let middle = as_string.len() / 2;
    let invalid = as_string[0..middle] == as_string[middle..];

    if invalid{
        println!("{} is not a valid number!", as_string);
    }
    invalid
}

// Returns the sum of the invalid ranges
fn process_range(start: i64, finish: i64) -> i64 {
    let mut sum = 0;
    for number in start..=finish {
        if is_invalid(number) {
            sum += number;
        }
    }

    sum
}
fn main() {
    // Get name of file
    let arguments: Vec<String> = env::args().collect();
    let file_name = arguments.get(1).expect("Please provide a file name!");

    // Read the file
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    let lines = contents
        .split(",")
        .collect::<Vec<&str>>();

    // Get the ranges
    let mut total = 0;
    for current_line in lines.iter() {
        let range = current_line.splitn(2, "-").collect::<Vec<&str>>();
        let start = i64::from_str(range[0]).unwrap();
        let finish = i64::from_str(range[1]).unwrap();

        total += process_range(start, finish);
    }

    println!("Invalid sum: {}", total);
}
