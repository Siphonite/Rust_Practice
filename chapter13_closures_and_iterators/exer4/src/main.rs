// Write code using .iter() and .next() manually to print all elements of a vector. 

fn main() {
    let numbers = vec![1, 2, 3, 4, 5]; // Sample vector of integers

    let mut iter = numbers.iter(); // Create an iterator over the vector

    // Use a loop to manually call .next() and print each element
    while let Some(num) = iter.next() {
        println!("{}", num); // Print the current element
    }
} 

