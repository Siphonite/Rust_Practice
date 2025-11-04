// Create a Box<i32> containing 5. Print the value inside using dereference * 

fn main() {
    let b = Box::new(5);  // Create a Box containing 5
    println!("{}", *b);
}