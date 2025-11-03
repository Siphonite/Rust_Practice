// Create a function divide(a: i32, b: i32) -> i32 that panics if b == 0. Call it with both safe and unsafe inputs. 

fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Division by zero is not allowed!");
    }
    a / b
} 

fn main() {
    let safe_result = divide(10, 2);
    println!("10 divided by 2 is {}", safe_result); // prints "10 divided by 2 is 5"

    // Uncommenting the line below will cause a panic
    // let unsafe_result = divide(10, 0);
    // println!("10 divided by 0 is {}", unsafe_result);
} 