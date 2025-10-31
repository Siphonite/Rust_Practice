// Take an integer input and print if it’s even or odd. 

use std::io; // Imports the io (input/output) module from the standard library.

fn main() {                                  // Defines the entry point of the program.
    println!("Please enter an integer:");    // Prompts the user to enter an integer.

    let mut input = String::new();           // Creates a mutable String variable to hold the user input.
    io::stdin()                              // Accesses the standard input stream.
        .read_line(&mut input)               // Reads a line from standard input and stores it in the 'input' variable.
        .expect("Failed to read line");      // Handles any potential errors during input reading.

    let number: i32 = match input.trim().parse() { // Trims whitespace and attempts to parse the input as a 32-bit integer.
        Ok(num) => num,                            // If parsing is successful, assigns the value to 'number'.
        Err(_) => {                                // If parsing fails, handles the error.
            println!("Please enter a valid integer."); // Informs the user of invalid input.
            return;                           // Exits the program early.
        }
    };

    if number % 2 == 0 {                      // Checks if the number is even.
        println!("The number {} is even.", number);  // Prints that the number is even.
    } else {                                  // If the number is not even, it must be odd.
        println!("The number {} is odd.", number); // Prints that the number is odd.
    }
}