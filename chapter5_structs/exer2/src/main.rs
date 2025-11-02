//  Write a function that takes a Color and prints its RGB values individually. 

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}   
fn print_rgb(color: &Color) {
    println!("Red: {}", color.red);
    println!("Green: {}", color.green);
    println!("Blue: {}", color.blue);
}  

fn main() {
    let my_color = Color {
        red: 255,
        green: 165,
        blue: 0,
    };

    print_rgb(&my_color);  // using a reference to avoid moving ownership
}