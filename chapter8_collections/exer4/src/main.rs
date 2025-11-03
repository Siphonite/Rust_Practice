// Create a HashMap<String, i32> of fruit names to quantities. Insert and print a few entries. 

use std::collections::HashMap;

fn main() {
    let mut fruit_quantities: HashMap<String, i32> = HashMap::new();

    // Inserting some fruit quantities
    fruit_quantities.insert("Apple".to_string(), 10);
    fruit_quantities.insert("Banana".to_string(), 20);
    fruit_quantities.insert("Orange".to_string(), 15);

    // Printing the fruit quantities
    for (fruit, quantity) in &fruit_quantities { // quantity is a reference
        println!("{}: {}", fruit, quantity);
    }
}