// Define a struct Rectangle with width and height. Write a method area(&self) -> u32 using impl 

struct Rectangle {
    width: u32,
    height: u32,
} 

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}    // Then create an instance of Rectangle and print its area using both the standalone function and the method.  

fn main() {
    let rect = Rectangle {
        width: 10,
        height: 5,
    };

    let area_function = area(&rect); // using standalone function
    let area_method = rect.area(); // 

    println!("Area using function: {}", area_function);
    println!("Area using method: {}", area_method);
}