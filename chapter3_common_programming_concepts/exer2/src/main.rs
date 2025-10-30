// Create a mutable variable count, increment it in a loop three times, and print the final value.

fn main() {
    let mut count = 0;
    for _ in 0..3 {
        count += 1;
        println!("Current count: {}", count);
    }
    println!("Final value of count: {}", count);
}