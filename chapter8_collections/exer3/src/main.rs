// Write a small program that takes a sentence and counts the number of words by splitting on spaces.

fn count_words(sentence: &str) -> usize {
    let words: Vec<&str> = sentence.split_whitespace().collect();
    words.len() // Returns the number of words
}

// Alternative: collect words into a single String (joined by spaces), then count
fn count_words_string(sentence: &str) -> usize {
    // Join all words with a single space, then split and count
    let joined: String = sentence.split_whitespace().collect::<Vec<&str>>().join(" ");
    joined.split_whitespace().count()
}

fn main() {
    let sentence1 = "This is a sample sentence for counting words.";
    let word_count1 = count_words(sentence1);
    println!("The sentence has {} words.", word_count1);

    let sentence2 = "This is another sample sentence for counting words. And this is a longer text.";
    let word_count2 = count_words_string(sentence2);
    println!("The sentence has {} words.", word_count2);
}