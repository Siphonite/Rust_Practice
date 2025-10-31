// Make a String, clone it into another variable, and print both. Compare the result to moving the String without cloning.

fn main() {
    let original = String::from("Hello, ownership!"); // Creating a String
    let cloned = original.clone(); // Cloning the String

    println!("Original: {}", original);
    println!("Cloned: {}", cloned);
}