//Try to change an immutable variable and fix the compile error.

fn main() {
    let mut x = 5; // Changed to mutable by adding 'mut'
    println!("Initial value of x: {}", x);
    x = 10; // Now this line works without a compile error
    println!("Changed value of x: {}", x);
}