// Define a generic function returns_summarizable() -> impl Summary that returns a struct implementing Summary. 

trait Summary {
    fn summarize(&self) -> String;
}   

struct NewsArticle {
    headline: String,
    location: String,
} 
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, located at {}", self.headline, self.location)
    }
} 
fn returns_summarizable() -> impl Summary {
    NewsArticle {
        headline: String::from("Breaking News!"),
        location: String::from("World"),
    }
}  

fn main() {
    let article = returns_summarizable();
    println!("Article Summary: {}", article.summarize());
}