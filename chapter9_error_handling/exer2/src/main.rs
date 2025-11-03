// Use std::fs::File::open("hello.txt") and match on the Result. Print “file found” or “file not found”. 

use std::fs::File; 
fn main() {
    match File::open("hello.txt") {
        Ok(_) => println!("file found"),
        Err(_) => println!("file not found"),
    }
} 
