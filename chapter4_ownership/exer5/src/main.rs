// Write a function get_first_char(s: &String) -> char that returns the first character without taking ownership. 

fn main() {
    let my_string = String::from("Hello, world!");
    let first_char = get_first_char(&my_string);
    println!("The first character is: {}", first_char);
}
fn get_first_char(s: &String) -> char {
    s.chars().next().unwrap()                       // Returns the first character of the string
}