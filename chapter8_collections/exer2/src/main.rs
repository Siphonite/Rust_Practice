// Create a mutable vector and iterate over it with for i in &mut v to modify each element (e.g., multiply by 2). 

fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    for i in &mut v {
        *i *= 2;
    }

    println!("{:?}", v); // prints [2, 4, 6, 8, 10]
}