//  Use iterators and closures together to compute the average of all positive numbers in a list — no manual loops, no mutable state.

fn main() {
    let numbers = vec![-10, 15, -20, 25, 30, -5]; // Sample vector of integers

    let positive_numbers: Vec<i32> = numbers       // Create a new vector with only positive numbers
        .into_iter()                              // Convert the vector into an iterator
        .filter(|&x| x > 0)                       // Filter to keep only positive numbers
        .collect();                               // Collect the results back into a vector

    let sum: i32 = positive_numbers.iter().sum(); // Sum the positive numbers
    let count: usize = positive_numbers.len();     // Count the number of positive numbers

    let average: f64 = if count > 0 {              // Calculate the average
        sum as f64 / count as f64
    } else {
        0.0
    };

    println!("Average of positive numbers: {}", average); // Print the average
}