//Declare let x = 10; in main, then a new x inside a block ({}) and print both values. 

fn main() {
    let x = 10;
    println!("Outside block, x: {}", x);

    {
        let x = 20; // Shadowing the outer x
        println!("Inside block, x: {}", x);
    }

    println!("Outside block again, x: {}", x);
}