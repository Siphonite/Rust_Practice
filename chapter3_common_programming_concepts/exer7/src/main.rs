// Create a tuple (String, i32, bool) and destructure it into variables, then print them. 

fn main() {
    let my_tuple: (String, i32, bool) = (String::from("Hello"), 42, true);

    let (my_string, my_integer, my_boolean) = my_tuple;

    println!("String value: {}", my_string);
    println!("Integer value: {}", my_integer);
    println!("Boolean value: {}", my_boolean);
}