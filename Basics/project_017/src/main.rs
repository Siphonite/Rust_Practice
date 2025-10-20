// Check if a hardcoded character is a vowel or consonant using match.

fn main() {
    let ch = 'a';

    match ch {
        'a' | 'e' | 'i' | 'o' | 'u' |
        'A' | 'E' | 'I' | 'O' | 'U' => println!("The character is a vowel."),
        _ => println!("The character is a consonant."),
    }
}   

