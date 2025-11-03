// Write a generic function print_vec<T: std::fmt::Debug>(v: Vec<T>) that prints elements of any vector using trait bounds. 

fn print_vec<T: std::fmt::Debug>(v: Vec<T>) { // Using trait bounds to ensure T implements Debug
    for item in v {
        println!("{:?}", item);
    }
}

fn main() {
    let int_vec = vec![1, 2, 3, 4, 5];
    let str_vec = vec!["hello", "world"];
    let float_vec = vec![1.1, 2.2, 3.3];

    print_vec(int_vec);   // prints integers
    print_vec(str_vec);   // prints strings
    print_vec(float_vec); // prints floats
}