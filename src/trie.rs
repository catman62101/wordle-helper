use std::{
    collections::{HashMap, HashSet},
    iter::Peekable,
    str::Chars,
};

use crate::util::has_letter_counts;

#[derive(Debug)]
pub struct Trie {
    children: HashMap<char, Trie>,
    is_end_of_word: bool,
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}

impl Trie {
    pub fn insert(&mut self, word: &str) {
        let mut node = Box::new(self);
        for c in word.chars() {
            let child = node.children.entry(c).or_default();
            node = Box::new(child);
        }
        node.is_end_of_word = true;
    }

    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
            is_end_of_word: false,
        }
    }

    pub fn from(words: Vec<String>) -> Self {
        let mut t = Self::default();
        for word in words {
            t.insert(&word);
        }
        t
    }

    pub fn words(&self) -> Vec<String> {
        let mut words_output = Vec::<String>::new();
        let mut stack = vec![(String::from(""), self)];

        while let Some((word, node)) = stack.pop() {
            stack.extend(
                node.children
                    .iter()
                    .map(|(letter, node)| (word.clone() + &letter.to_string(), node))
                    .collect::<Vec<(String, &Trie)>>(),
            );
            if node.is_end_of_word {
                words_output.push(word);
            }
        }

        words_output
    }

    // TODO: implement pruning logic for pruned copy

    pub fn pruned_copy(
        &self,
        mut correct_pattern_iter: Peekable<Chars>,
        mut misplaced_pattern_iter: Peekable<Chars>,
        mut incorrect_pattern_iter: Peekable<Chars>,
        letter_counts: &HashMap<char, isize>,
        missing_letters: &HashSet<char>,
        word: String,
    ) -> Option<Self> {
        if self.is_end_of_word {
            if !has_letter_counts(word.as_str(), letter_counts) {
                return None;
            }
            return Some(Trie {
                children: Default::default(),
                is_end_of_word: true,
            });
        }
        let correct_letter = match correct_pattern_iter.next() {
            Some(letter) => letter,
            None => return None,
        };
        let misplaced_letter = match misplaced_pattern_iter.next() {
            Some(letter) => letter,
            None => return None,
        };
        let incorrect_letter = match incorrect_pattern_iter.next() {
            Some(letter) => letter,
            None => return None,
        };

        let return_node = match correct_letter {
            '*' => {
                let mut new_node = Trie::default();
                self.children
                    .iter()
                    .filter(|(letter, _)| {
                        // TODO: make missing_letters into a bitset
                        **letter != misplaced_letter
                            && **letter != incorrect_letter
                            && !missing_letters.contains(*letter)
                    })
                    .for_each(|(letter, child)| {
                        if let Some(child) = child.pruned_copy(
                            correct_pattern_iter.clone(),
                            misplaced_pattern_iter.clone(),
                            incorrect_pattern_iter.clone(),
                            letter_counts,
                            missing_letters,
                            word.clone() + &letter.to_string(),
                        ) {
                            new_node.children.insert(*letter, child);
                        }
                    });
                Some(new_node)
            }
            letter => {
                let mut new_node = Trie::default();

                if let Some(child) = self.children.get(&letter) {
                    if let Some(child_copy) = child.pruned_copy(
                        correct_pattern_iter.clone(),
                        misplaced_pattern_iter.clone(),
                        incorrect_pattern_iter.clone(),
                        letter_counts,
                        missing_letters,
                        word + &letter.to_string(),
                    ) {
                        new_node.children.insert(letter, child_copy);
                    }
                }

                Some(new_node)
            }
        };

        return_node
    }
}

#[cfg(test)]
mod tests {
    use crate::util::get_letter_counts;

    use super::*;

    #[test]
    fn test_insert() {
        let mut words = vec!["apple", "ape", "app", "candy", "return"];
        words.sort();
        let mut t = Trie::new();
        words.iter().for_each(|word| t.insert(word));
        let mut outputted_words = t.words();
        outputted_words.sort();
        assert_eq!(words, outputted_words);
    }

    #[test]
    fn test_pruned_copy() {
        let words = vec!["apple", "atria", "atoms"];
        let mut t = Trie::new();
        words.iter().for_each(|word| t.insert(word));
        let correct_letters = "at***";
        let misplaced_letters = "*****";
        let incorrect_letters = "****s";
        let pruned = t
            .pruned_copy(
                correct_letters.chars().peekable(),
                misplaced_letters.chars().peekable(),
                incorrect_letters.chars().peekable(),
                &get_letter_counts(correct_letters, misplaced_letters),
                &mut Default::default(),
                String::from(""),
            )
            .unwrap();
        let pruned_words = pruned.words();
        assert_eq!(pruned_words, vec![String::from("atria")]);
    }
}
