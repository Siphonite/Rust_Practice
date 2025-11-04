//  Chain multiple iterator adapters (map, filter, collect) to transform a list of numbers into a list of squares of even numbers. 

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; // Sample vector of integers

    let squares_of_even_numbers: Vec<i32> = numbers       // Create a new vector with squares of even numbers
        .into_iter()                                     // Convert the vector into an iterator
        .filter(|&x| x % 2 == 0)                         // Filter to keep only even numbers
        .map(|x| x * x)                                 // Map each even number to its square
        .collect();                                      // Collect the results back into a vector

    println!("Squares of even numbers: {:?}", squares_of_even_numbers); // Print the resulting vector
}