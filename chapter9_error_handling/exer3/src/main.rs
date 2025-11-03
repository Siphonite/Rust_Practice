// Create a function parse_number_from_file() -> Result<i32, Box<dyn Error>> that reads a number from a text file and converts it with .parse::<i32>(). Handle both I/O and parsing errors. 

use std::error::Error;
use std::fs::File;
use std::io::Read;

fn parse_number_from_file() -> Result<i32, Box<dyn Error>> {
    let mut file = File::open("number.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let number: i32 = contents.trim().parse()?;
    Ok(number)
} 
fn main() {
    match parse_number_from_file() {
        Ok(num) => println!("The number is: {}", num),
        Err(e) => println!("Error occurred: {}", e),
    } 

    println!("Program continues running after error handling."); 
}