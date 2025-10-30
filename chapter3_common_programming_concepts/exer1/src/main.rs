// Declare a variable x = 5, then shadow it twice — first by doubling it, then by converting it to a string ("10"). Print each stage.

fn main() {
    let x = 5;
    println!("Initial value of x: {}",x);

    let x = x*2;
    println!("After doubling, x: {}",x);    

    let x = x.to_string();
    println!("After converting to string, x: {}",x);
}