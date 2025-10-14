// 4. Convert a hardcoded Celsius temperature to Fahrenheit and print the result. Also converting a different hardcoded Fahrenheit temperature 
fn main() {
    let celsius: f32 = 25.0;
    let farhenite: f32 = (celsius * 9.0 /5.0) + 32.0;
    println!("{}°C is equal to {}°F", celsius, farhenite);

    let n_farhenite: f32 = 77.0;
    let n_celsius: f32 = (n_farhenite - 32.0* 5.0) / 9.0;
    println!("{}°F is equal to {}°C", n_farhenite, n_celsius);
}