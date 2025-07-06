use std::io::{BufRead, BufReader, Read};

use crate::{trie::Trie, util::get_letter_counts};

pub struct Dictionary {
    trie: Option<Trie>,
}

impl Dictionary {
    // TODO: Make this insert into a trie directly
    pub fn read_from_file<T: Read>(reader: T) -> Option<Self> {
        let buf = BufReader::new(reader);
        let words = buf
            .lines()
            .flatten()
            .filter(|word| word.len() == 5)
            .map(|word| word.to_lowercase())
            .collect::<Vec<String>>();
        if words.is_empty() {
            return None;
        }
        Some(Dictionary::from(words))
    }

    pub fn from(words: Vec<String>) -> Self {
        Self {
            trie: Some(Trie::from(words)),
        }
    }

    pub fn words(&self) -> Vec<String> {
        match &self.trie {
            Some(trie) => trie.words(),
            None => vec![],
        }
    }

    pub fn filter_words(
        &mut self,
        correct_letters: &str,
        misplaced_letters: &str,
        missing_letters: &str,
    ) {
        self.trie = match &self.trie {
            Some(trie) => trie.pruned_copy(
                correct_letters.chars().peekable(),
                misplaced_letters.chars().peekable(),
                &mut get_letter_counts(correct_letters, misplaced_letters),
                missing_letters,
                String::from(""),
            ),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn load_clean_dictionary() {
        let words = vec!["apple", "words", "catty"]
            .into_iter()
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();
        let words_dump = Cursor::new(words.join("\n"));
        let dictionary = Dictionary::read_from_file(words_dump).unwrap();
        assert_eq!(dictionary.words(), words);
    }

    #[test]
    fn load_unfiltered_dictionary() {
        let clean_words = vec!["apple", "words", "catty"]
            .into_iter()
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();
        let unfiltered_words = vec!["cat", "dog", "arachnophobia"]
            .into_iter()
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();
        let unnormalized_words = vec!["grApe", "dOGGY", "SPIce"]
            .into_iter()
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();
        let words_dump = Cursor::new(
            [
                clean_words.clone(),
                unnormalized_words.clone(),
                unfiltered_words,
            ]
            .concat()
            .join("\n"),
        );
        let dictionary = Dictionary::read_from_file(words_dump).unwrap();
        assert_eq!(
            dictionary.words(),
            [
                clean_words,
                // Dictionary should load words in lowercase
                unnormalized_words
                    .into_iter()
                    .map(|s| s.to_lowercase())
                    .collect()
            ]
            .concat()
        );
    }
}
