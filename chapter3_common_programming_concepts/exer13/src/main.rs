// Create a function that takes two numbers and returns both their sum and difference as a tuple. 

fn main() {
    let a = 10;
    let b = 5;
    let (sum, difference) = calculate(a, b);

    println!("The sum is: {}", sum);
    println!("The difference is: {}", difference);
}

fn calculate(x: i32, y: i32) -> (i32, i32) {
    let sum = x + y;
    let difference = x - y;
    (sum, difference)  // This is the return value
}
