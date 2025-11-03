// Write a function longest<'a>(x: &'a str, y: &'a str) -> &'a str that returns the longer string slice. 

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { // specify lifetime 'a for the string slices
    if x.len() > y.len() {
        x
    } else {
        y
    }
} 
fn main() {
    let string1 = String::from("long string");
    let string2 = "short";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is: {}", result);
} 
