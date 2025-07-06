use std::{
    collections::HashSet,
    io::{self, BufRead, Write},
};

use crate::{
    dictionary::Dictionary,
    util::{update_missing_letters, validate_input},
};

pub fn repl(dictionary: &mut Dictionary) {
    let mut missing_letters = HashSet::new();
    loop {
        let correct_letters = read_line_from_user("correct");
        if correct_letters == ".exit" {
            break;
        }
        let misplaced_letters = read_line_from_user("misplaced");
        if misplaced_letters == ".exit" {
            break;
        }
        let incorrect_letters = read_line_from_user("incorrect");
        if incorrect_letters == ".exit" {
            break;
        }
        let valid = [
            validate_input(correct_letters.as_str(), "correct"),
            validate_input(misplaced_letters.as_str(), "misplaced"),
            validate_input(incorrect_letters.as_str(), "incorrect"),
        ]
        .iter()
        .all(|result| match result {
            Ok(()) => true,
            Err(why) => {
                println!("{why}");
                false
            }
        });
        if !valid {
            continue;
        }
        update_missing_letters(incorrect_letters.as_str(), &mut missing_letters);
        dictionary.filter_words(
            correct_letters.as_str(),
            misplaced_letters.as_str(),
            incorrect_letters.as_str(),
            &missing_letters,
        );
        for word in dictionary.words() {
            println!("{word}");
        }
    }
}

fn read_line_from_user(prompt: &str) -> String {
    print!("{prompt} > ");
    io::stdout().flush().unwrap();
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();
    line.trim().to_owned()
}
