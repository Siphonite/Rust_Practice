// 16. Use a match expression to print a message based on a hardcoded number (1, 2, or other). 

fn main(){
    let x = 2;

    match x {
        1 => println!("The number is one."),
        2 => println!("The number is two."),
        _ => println!("The number is something else."),
    }
}