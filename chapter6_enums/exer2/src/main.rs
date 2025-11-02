// Write a function describe_number(n: Option<i32>) that prints “positive”, “zero”, “negative”, or “no number” using match. 

fn describe_number(n: Option<i32>) {
    match n {
        Some(0) => println!("zero"),
        Some(num) => {
            if num > 0 {
                println!("positive");
            } else {
                // num must be < 0 here because 0 was handled above
                println!("negative");
            }
        }
        None => println!("no number"),
    }
}

fn main() {
    describe_number(Some(10));  // prints "positive"
    describe_number(Some(0));   // prints "zero"
    describe_number(Some(-5));  // prints "negative"
    describe_number(None);      // prints "no number"
}