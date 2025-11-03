// Define a trait Summary with a method summarize(&self) -> String. Implement it for two structs: NewsArticle and Tweet. 

trait Summary { // define the Summary trait
    fn summarize(&self) -> String;
} 
struct NewsArticle {
    headline: String,
    location: String,
}
struct Tweet {
    username: String,
    content: String,
}
impl Summary for NewsArticle { // implement the Summary trait for NewsArticle
    fn summarize(&self) -> String {
        format!("{}, located at {}", self.headline, self.location)
    }
}
impl Summary for Tweet {  // implement the Summary trait for Tweet
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Rust is awesome!"),
        location: String::from("Internet"),
    };
    let tweet = Tweet {
        username: String::from("user123"),
        content: String::from("Learning Rust traits!"),
    };

    println!("Article Summary: {}", article.summarize());
    println!("Tweet Summary: {}", tweet.summarize());
}