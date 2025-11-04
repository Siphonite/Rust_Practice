// Write a test using assert!(condition, "Custom message: {:?}", value) to print context on failure. 

fn is_even(n: i32) -> bool {
    n % 2 == 0
} 
#[cfg(test)]
mod tests {
    use super::*;       
    #[test]
    fn test_is_even() {
        let value = 3;
        assert!(is_even(value), "Expected {} to be even", value);
    }
}
fn main() {
    let number = 545;
    if is_even(number) {
        println!("{} is even", number);
    } else {
        println!("{} is odd", number);
    }
}