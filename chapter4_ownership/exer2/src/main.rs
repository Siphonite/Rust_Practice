// Create a function makes_copy(x: i32) that prints x. Call it twice with the same integer and note why this works.


fn main() {
    let a = 5;
    makes_copy(a);
    makes_copy(a);
}

fn makes_copy(x: i32) {
    println!("The value of x is: {}", x);
}