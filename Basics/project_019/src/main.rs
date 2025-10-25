// Create a simple calculator using match to perform addition, subtraction, multiplication, or division on two hardcoded numbers based on a hardcoded operator.

fn main() {
    // Hardcoded numbers and operator
    let num1: f64 = 10.0;
    let num2: f64 = 5.0;
    let operator: char = '/';

    // Perform calculation using match
    let result = match operator {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => {
            if num2 != 0.0 {
                num1 / num2
            } else {
                println!("Error: Division by zero!");
                return;
            }
        }
        _ => {
            println!("Error: Invalid operator!");
            return;
        }
    };

    // Print the result
    println!("{} {} {} = {}", num1, operator, num2, result);
}