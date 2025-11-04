// Build a closure that filters a Vec<i32> and keeps only even numbers. 

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];   // Sample vector of integers

    let even_numbers: Vec<i32> = numbers          // Create a new vector with only even numbers
        .into_iter()                              // Convert the vector into an iterator
        .filter(|&x| x % 2 == 0)                  // Use a closure to filter even numbers
        .collect();                               // Collect the results back into a vector

    println!("Even numbers: {:?}", even_numbers); // Print the resulting vector of even numbers
}