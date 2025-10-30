// Write a small program that calculates the average of an array of i32.

fn main() {
    // Create an array of i32 values
    let numbers: [i32; 5] = [10, 20, 30, 40, 50];

    // Calculate the sum of the elements in the array
    let sum: i32 = numbers.iter().sum();

    // Calculate the average
    let average: f32 = sum as f32 / numbers.len() as f32;

    // Print the average
    println!("The average is: {}", average);
}