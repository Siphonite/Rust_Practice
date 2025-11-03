// Create a new vector of integers using both Vec::new() and the macro vec![]. Print both. 

fn main() {
    // Creating a vector using Vec::new()
    let mut vec1: Vec<i32> = Vec::new();
    vec1.push(1);
    vec1.push(2);
    vec1.push(3);
    println!("Vector created with Vec::new(): {:?}", vec1);

    // Creating a vector using the vec! macro
    let vec2 = vec![4, 5, 6];
    println!("Vector created with vec! macro: {:?}", vec2);
}