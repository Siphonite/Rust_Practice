//Write a function greet(name: &str) that prints "Hello, <name>". 

fn main() {
    let name = "Alice";
    greet(name);
} 

fn greet(name: &str) {
    println!("Hello, {}", name);
}