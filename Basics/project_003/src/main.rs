// 3. Declare two integer variables, add them, and print the result. 

fn main() {
    let a:i32 = 468;
    let b:i32 = 254;
    let c = a+b; 
    println!("The sum of and b is {}",c); 

    let x:i32 = 412;
    let y:f32 = 475.365;
    let z = a as f32 + b as f32 + x as f32 + y; 
    println!("The sum of all variables is {}", z);
}