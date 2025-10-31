// Write a function make_uppercase(s: &mut String) that appends "!" to the string. Pass a mutable reference and print the modified string. 

       
fn main() {
    let mut my_string = String::from("hello"); // Creating a mutable String
    make_uppercase(&mut my_string); // Passing a mutable reference to the function
    println!("{}", my_string); // Printing the modified string
} 

fn make_uppercase(s: &mut String) {
    s.push('!'); // Appending "!" to the string
}