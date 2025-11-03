// Make another struct Point<T, U> where x and y can have different types (e.g., one i32, one f64) 

struct Point<T, U> {
    x: T,
    y: U,
} 

fn main() {
    let integer_point = Point { x: 5, y: 10.0 }; // x is i32, y is f64
    let float_point = Point { x: 3.5, y: 7 };    // x is f64, y is i32

    println!("Integer Point: ({}, {})", integer_point.x, integer_point.y);
    println!("Float Point: ({}, {})", float_point.x, float_point.y);
}