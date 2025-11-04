// Write a function takes_ownership(s: String) that prints the string. Call it with a String from main and then try printing the same variable again. // Fix the code so that it works.
fn main() {
    let s = String::from("hello"); // s comes into scope

    let s = takes_ownership(s);  // s's value moves into the function and then is returned back to main

    println!("Back in main: {s}"); // s can still be used here because ownership was returned
}

fn takes_ownership(s: String) -> String {   // defines a function that takes ownership of a String and returns it
    println!("Inside takes_ownership: {s}"); // prints the string
    s
}