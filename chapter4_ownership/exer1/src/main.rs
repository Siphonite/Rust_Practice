// Write a function takes_ownership(s: String) that prints the string. Call it with a String from main and then try printing the same variable again. // Fix the code so that it works.
fn main() {
    let s = String::from("hello");

    let s = takes_ownership(s);

    println!("Back in main: {s}");
}

fn takes_ownership(s: String) -> String {
    println!("Inside takes_ownership: {s}");
    s
}