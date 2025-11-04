// Write a function apply_twice<F>(f: F, x: i32) -> i32 that takes a closure and applies it twice to a number. 

fn apply_twice<F>(f: F, x: i32) -> i32 // Define the function with a generic type F
where
    F: Fn(i32) -> i32,   // Specify that F is a closure that takes an i32 and returns an i32
{
    f(f(x))   // Apply the closure f to x twice
} 

fn main() {
    let add_three = |n: i32| n + 3;  // Define a closure that adds 3 to its input

    let result = apply_twice(add_three, 7); // Call apply_twice with the closure and the number 7
    println!("Result: {}", result); // prints "Result: 13"
}