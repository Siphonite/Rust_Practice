// Design a small program that defines a Book struct with title, author, and price. Write an impl with a method discount(&mut self, percent: f32) that reduces the price. Instantiate a few books, apply discounts, and print the updated details. 

struct Book {
    title: String,
    author: String,
    price: f32,
} 

impl Book {
    fn discount(&mut self, percent: f32) {
        if percent < 0.0 || percent > 100.0 {
            println!("Invalid discount percentage: {}. It must be between 0 and 100.", percent);
            return;
        }
        let discount_amount = self.price * (percent / 100.0);
        self.price -= discount_amount;
    }
}

fn main() {
    let mut book1 = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
        price: 39.99,
    }; 
    let mut book2 = Book {
        title: String::from("Programming in Rust"),
        author: String::from("Jim Blandy and Jason Orendorff"),
        price: 49.99,
    };
    let mut book3 = Book {
        title: String::from("Rust by Example"),
        author: String::from("Various Authors"),
        price: 29.99,
    };
    book1.discount(10.0);
    book2.discount(15.0);
    book3.discount(5.0);    
    println!("Updated Book Details:");
    println!("Title: {}, Author: {}, Price: ${:.2}", book1.title, book1.author, book1.price);
    println!("Title: {}, Author: {}, Price: ${:.2}", book2.title, book2.author, book2.price);
    println!("Title: {}, Author: {}, Price: ${:.2}", book3.title, book3.author, book3.price); 

}