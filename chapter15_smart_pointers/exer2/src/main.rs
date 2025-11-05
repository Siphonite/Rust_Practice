// Write a function print_box(b: &Box<i32>) that borrows the box and prints the inner value without taking ownership. 

fn print_box(b: &Box<i32>) {
    println!("{}", **b);
} 

fn main() {
    let my_box = Box::new(42);
    print_box(&my_box); // prints "42"
}