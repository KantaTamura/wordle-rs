use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::seq::IteratorRandom;

pub struct WordList {
    all_words: HashSet<String>,
}

impl WordList {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut all_words = HashSet::new();
        const WORD_FILE: &str = "./wordle-list/words";
        for line in BufReader::new(File::open(WORD_FILE)?).lines() {
            let line = line?;
            all_words.insert(line);
        }
        Ok(WordList { all_words })
    }

    pub fn get_random_word(&self) -> Result<&str, Box<dyn std::error::Error>> {
        Ok(self.all_words.iter().choose(&mut rand::thread_rng()).unwrap())
    }

    pub fn is_valid(&self, word: &str) -> bool {
        if word.len() != 5 { return false; }
        self.all_words.contains(word)
    }
}

#[test]
fn test_check_word() -> Result<(), Box<dyn std::error::Error>> {
    let words = WordList::new()?;
    assert!(words.is_valid("label"));
    assert!(words.is_valid("hello"));
    assert!(!words.is_valid("four"));
    assert!(!words.is_valid("abcde"));
    Ok(())
}

#[test]
fn test_select_random_word() -> Result<(), Box<dyn std::error::Error>> {
    let words = WordList::new()?;
    let word = words.get_random_word()?;
    assert!(words.is_valid(word));
    Ok(())
}