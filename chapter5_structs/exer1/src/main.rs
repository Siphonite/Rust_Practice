// Define a struct User with fields username, email, sign_in_count, and active. Create an instance and print each field. 

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}   

fn main() {
    let user1 = User {
        username: String::from("Aman"),
        email: String::from("aman12345@gamil.com"),
        sign_in_count: 1,
        active: true, 
    };
    
println!("Username: {}", user1.username);
    println!("Email: {}", user1.email);
    println!("Sign-in count: {}", user1.sign_in_count);
    println!("Active: {}", user1.active);
}