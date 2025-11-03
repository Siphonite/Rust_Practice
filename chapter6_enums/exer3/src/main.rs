// Define an enum Shape with variants Circle(f64), Rectangle(f64, f64), and Triangle(f64, f64, f64). Write a function area(shape: &Shape) -> f64 using match to calculate the area for each.


enum Shape{ 
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64),
}
fn area(s: &Shape) -> f64 {
    match s {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Rectangle(width, height) => width * height,
        Shape::Triangle(base, height) => 0.5 * base * height,
    }
} 

fn main() {
    let c = Shape::Circle(5.0);
    let r = Shape::Rectangle(4.0, 6.0);
    let t = Shape::Triangle(3.0, 4.0);

    println!("Area of Circle: {}", area(&c));
    println!("Area of Rectangle: {}", area(&r));
    println!("Area of Triangle: {}", area(&t));
}