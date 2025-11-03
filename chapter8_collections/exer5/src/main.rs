// Write a word counter: given a string, split it into words and use a HashMap to count how many times each appears. Print the counts. 

use std::collections::HashMap; 

fn word_counter(text: &str) {
    let mut word_counts: HashMap<&str, i32> = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_counts.entry(word).or_insert(0);
        *count += 1;
    }

    for (word, count) in &word_counts {
        println!("{}: {}", word, count);
    }
} 

fn main() {
    let text = "hello world hello rust rust programming world";
    word_counter(text);
} 
