// Create a small command-line program that reads an integer from input. Use match to handle both valid and invalid input without panicking. Print helpful messages rather than stack traces. 

use std::io; 

fn main() {
    println!("Please enter an integer:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().parse::<i32>() {
        Ok(num) => println!("You entered the integer: {}", num),
        Err(_) => println!("That's not a valid integer. Please try again."),
    }
}