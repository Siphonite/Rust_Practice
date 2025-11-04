// Write a function add(a: i32, b: i32) -> i32 and a simple test that asserts add(2, 2) equals 4. 

fn add(a: i32, b: i32) -> i32 {
    a + b
} 

#[cfg(test)]   // module for tests
mod tests {    // import everything from the outer scope
    use super::*;   // bring add function into scope
    #[test]     // define a test function
    fn test_add() {  // test for add function
        assert_eq!(add(2, 2), 4);  // assert that add(2, 2) equals 4
    }
}
fn main() {
    let sum = add(5, 7);
    println!("The sum of 5 and 7 is: {}", sum);
}