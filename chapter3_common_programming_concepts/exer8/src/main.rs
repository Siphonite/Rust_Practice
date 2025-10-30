// Store 5 integers in an array, access the third one, and change the last element. 

fn main() {
    // Create an array of 5 integers
    let mut numbers = [10, 20, 30, 40, 50];

    // Access the third element (index 2)
    let third_element = numbers[2];
    println!("The third element is: {}", third_element);

    // Change the last element (index 4)
    numbers[4] = 100;
    println!("The modified array is: {:?}", numbers);
}