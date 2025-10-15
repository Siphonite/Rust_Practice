//13. Calculate the sum of numbers from 1 to 10 using a for loop. 

fn main() {

let mut sum = 0; // declare a mutable variable to hold the sum

 for x in 1..11 { // iterate from 1 to 10
   sum = sum + x;   // add each number to the sum   
 }
 println!("The sum of all numbers is {}",sum);
 }
 