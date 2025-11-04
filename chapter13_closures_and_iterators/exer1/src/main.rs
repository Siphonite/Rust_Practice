// Write a closure that takes two numbers and returns their sum. Call it inline inside main(). 

fn main() {
    let sum = |a: i32, b: i32| a + b;

    let result = sum(5, 7);
    println!("The sum is: {}", result); // prints "The sum is: 12"
} 
