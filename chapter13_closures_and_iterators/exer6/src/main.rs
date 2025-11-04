 //  Combine iterators: take a Vec<String> and use .into_iter().zip() with a Vec<i32> to produce tuples of (String, i32). 

fn main() {
    let strings = vec![
        String::from("apple"),
        String::from("banana"),
        String::from("cherry"),
    ]; // Sample vector of strings

    let numbers = vec![1, 2, 3]; // Sample vector of integers

    let zipped: Vec<(String, i32)> = strings
        .into_iter()               // Convert the vector of strings into an iterator
        .zip(numbers.into_iter())  // Zip it with the iterator of integers
        .collect();                // Collect the results into a vector of tuples

    println!("Zipped tuples: {:?}", zipped); // Print the resulting vector of tuples
}