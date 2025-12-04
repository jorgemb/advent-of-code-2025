use std::{env, fs};

fn main() {
    // Get name of file
    let arguments : Vec<String> = env::args().collect();
    let file_name = arguments.get(1)
        .expect("Please provide a file name!");

    // Read the file
    let contents = fs::read_to_string(file_name)
        .expect("Something went wrong reading the file");
    let rotations = contents.lines().collect::<Vec<&str>>();

    // Count rotations -- First method
    const MAX_COUNTER: i32 = 100;
    let mut counter = 50;
    let mut zeros = 0;
    let mut passed_zeros = 0;
    for current_rotation in rotations.iter() {
        let left_rotation = current_rotation.starts_with("L");
        let number = current_rotation[1..].parse::<i32>().unwrap();

        // Change rotation
        if left_rotation {
            counter -= number;
            while counter < 0{
                // Handle special case when counter started in zero
                if counter + number != 0{
                    passed_zeros += 1;
                }
                counter += MAX_COUNTER;
            }

            // On left, zero requires extra count
            if counter == 0{
                passed_zeros += 1;
            }

        } else {
            counter += number;
            passed_zeros += counter / MAX_COUNTER;
            counter %= MAX_COUNTER;
        }

        if counter == 0{
            zeros += 1;
        }
    }

    println!("{}", zeros);
    println!("{}", passed_zeros);
}
