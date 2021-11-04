use std::io::{self, Write};
use std::collections::HashMap;

pub fn letter() {
    print!("Please enter a word: ");
    io::stdout().flush().ok().expect("Could not flush stdout.");
    let mut word = String::new();
    io::stdin().read_line(&mut word).expect("Failed to read line.");
    let mut map = HashMap::new();

    for letter in word.trim().chars() {
        let count = map.entry(letter).or_insert(0);
        *count += 1;
    }
    println!("Frequency of words in {} is {:#?}", word, map);
}