// Define an enum IpAddrKind with two variants: V4 and V6. Create one value of each and print them using println!. 

#[derive(Debug)]

enum IpAddrKind {
    V4,
    V6,
}
fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("IP Address Kind 1: {:?}", four);
    println!("IP Address Kind 2: {:?}", six);
}