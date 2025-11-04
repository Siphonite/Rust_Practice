// Define a struct ImportantExcerpt<'a> that holds a reference &'a str and implement a method that prints the part. 

struct ImportantExcerpt<'a> {
    part: &'a str,
} 

impl<'a> ImportantExcerpt<'a> {
    fn print_part(&self) {
        println!("Excerpt part: {}", self.part);
    }
} 

fn main() {
    let text = String::from("Rust is a systems programming language focused on safety and performance.");
    let first_sentence = text.split('.').next().expect("Could not find a '.'");
    
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    
    excerpt.print_part();
} 

