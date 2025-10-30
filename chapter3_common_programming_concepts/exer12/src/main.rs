// Write a function square(num: i32) -> i32 and call it. 

fn main() {
    let num = 4;
    let result = square(num);
    println!("The square of {} is {}", num, result);
}       

 fn square(num: i32) -> i32 {
    num * num
}