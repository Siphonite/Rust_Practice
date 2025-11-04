// Create a new tests directory and write an integration test file tests/integration_test.rs.

use my_mini_grep::{search, search_case_insensitive};
use std::fs;

#[test]
fn test_search_basic() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    let results = search(query, contents);
    assert_eq!(results, vec!["safe, fast, productive."]);
}

#[test]
fn test_search_multiple_matches() {
    let query = "the";
    let contents = "\
The quick brown fox
jumps over the lazy dog.
The end.";

    let results = search(query, contents);
    assert_eq!(results, vec!["jumps over the lazy dog."]);
}

#[test]
fn test_search_no_matches() {
    let query = "rust";
    let contents = "\
Python is a language.
JavaScript too.";

    let results = search(query, contents);
    assert_eq!(results.len(), 0);
}

#[test]
fn test_search_case_sensitive() {
    let query = "Rust";
    let contents = "\
Rust:
safe, fast, productive.
Trust me.";

    let results = search(query, contents);
    assert_eq!(results, vec!["Rust:"]);
}

#[test]
fn test_search_case_insensitive_basic() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    let results = search_case_insensitive(query, contents);
    assert_eq!(results, vec!["Rust:", "Trust me."]);
}

#[test]
fn test_search_case_insensitive_multiple() {
    let query = "tO";
    let contents = "\
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!";

    let results = search_case_insensitive(query, contents);
    assert_eq!(
        results,
        vec![
            "Are you nobody, too?",
            "How dreary to be somebody!",
            "To tell your name the livelong day",
            "To an admiring bog!"
        ]
    );
}

#[test]
fn test_search_empty_query() {
    let query = "";
    let contents = "Some content here.";

    let results = search(query, contents);
    // Every line contains an empty string
    assert_eq!(results, vec!["Some content here."]);
}

#[test]
fn test_search_empty_contents() {
    let query = "test";
    let contents = "";

    let results = search(query, contents);
    assert_eq!(results.len(), 0);
}

#[test]
fn test_search_with_file() {
    // Create a temporary test file
    let test_file = "test_poem.txt";
    let contents = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.";

    fs::write(test_file, contents).expect("Unable to write test file");

    // Read the file and search
    let file_contents = fs::read_to_string(test_file).expect("Unable to read test file");
    let results = search("nobody", &file_contents);

    assert_eq!(results, vec!["I'm nobody! Who are you?", "Are you nobody, too?"]);

    // Clean up
    fs::remove_file(test_file).expect("Unable to delete test file");
}

#[test]
fn test_search_special_characters() {
    let query = "c++";
    let contents = "\
I like c++ programming.
Python is great.
c++ is powerful.";

    let results = search(query, contents);
    assert_eq!(results, vec!["I like c++ programming.", "c++ is powerful."]);
}

#[test]
fn test_search_with_whitespace() {
    let query = "  test  ";
    let contents = "\
This is a   test   line.
Another line here.
  test  is here too.";

    let results = search(query, contents);
    // The search function looks for the exact query string including spaces
    // Both lines contain "  test  " (with surrounding spaces)
    assert_eq!(results, vec!["This is a   test   line.", "  test  is here too."]);
}

#[test]
fn test_multiline_search() {
    let query = "Rust";
    let contents = "\
Rust is a systems programming language.
It focuses on safety and performance.
Rust has zero-cost abstractions.
Many developers love Rust.";

    let results = search(query, contents);
    assert_eq!(
        results,
        vec![
            "Rust is a systems programming language.",
            "Rust has zero-cost abstractions.",
            "Many developers love Rust."
        ]
    );
}