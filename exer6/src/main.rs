// Write a function first_word(s: &String) -> &str that loops through the string and returns the first word slice up to the first space.

fn main() {
    let my_string = String::from("Hello, world!");
    let word = first_word(&my_string);
    println!("The first word is: {}", word);
} 

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes(); // Convert the string to bytes for iteration

    for (i, &item) in bytes.iter().enumerate() { // Loop through each byte with its index
        if item == b' ' {          // Check for space character
            return &s[0..i]; // Return the slice from start to the first space
        }
    }

    &s[..] // If no space is found, return the whole string
}