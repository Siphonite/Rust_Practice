// 10. Swap the values of two variables using tuple destructuring. 

fn main() {
    let mut a = 5; 
    let mut b = 35;
    println!("Before swap the value of a is {} and b is {}", a, b);
    (a, b) = (b, a);
    println!("After swap the value of a is {} and b is {}", a, b);
}