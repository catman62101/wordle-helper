use std::collections::{HashMap, HashSet};

pub fn validate_input(input: &str, name: &str) -> Result<(), String> {
    if input.len() != 5 || !input.chars().all(|c| c.is_alphabetic() || c == '*') {
        return Err(invalid_format_msg(name));
    }
    Ok(())
}

fn invalid_format_msg(name: &str) -> String {
    format!("Input `{name}` has incorrect format")
}

pub fn get_letter_counts(correct_letters: &str, misplaced_letters: &str) -> HashMap<char, isize> {
    let mut counts = HashMap::<char, isize>::new();
    correct_letters
        .chars()
        .chain(misplaced_letters.chars())
        .filter(|c| *c != '*')
        .for_each(|c| *counts.entry(c).or_insert(0) += 1);
    counts
}

pub fn has_letter_counts(word: &str, letter_counts: &HashMap<char, isize>) -> bool {
    let mut counts = HashMap::<char, isize>::new();
    word.chars()
        .filter(|c| letter_counts.contains_key(c))
        .for_each(|c| *counts.entry(c).or_insert(0) += 1);
    letter_counts
        .iter()
        .all(|(c, count)| count <= counts.get(c).unwrap_or(&0_isize))
}

pub fn update_missing_letters(incorrect: &str, missing: &mut HashSet<char>) {
    incorrect.chars().filter(|c| *c != '*').for_each(|c| {
        missing.insert(c);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_input() {
        struct TestCase<'a> {
            name: &'a str,
            input: String,
            input_name: &'a str,
            expected: Result<(), String>,
        }

        let test_cases = vec![
            TestCase {
                name: "valid format",
                input: String::from("*f*f*"),
                input_name: "correct",
                expected: Ok(()),
            },
            TestCase {
                name: "invalid format with numbers",
                input: String::from("12345"),
                input_name: "correct",
                expected: Err(invalid_format_msg("correct")),
            },
            TestCase {
                name: "invalid format, input too long",
                input: String::from("asdfgh"),
                input_name: "correct",
                expected: Err(invalid_format_msg("correct")),
            },
            TestCase {
                name: "invalid format, input too short",
                input: String::from("asdf"),
                input_name: "correct",
                expected: Err(invalid_format_msg("correct")),
            },
        ];

        test_cases.iter().for_each(|tc| {
            println!("{}", tc.name);
            assert_eq!(validate_input(&tc.input, tc.input_name), tc.expected)
        })
    }

    #[test]
    fn test_get_letter_counts() {
        assert_eq!(
            get_letter_counts("*abc*", "**def"),
            HashMap::from([('a', 1), ('b', 1), ('c', 1), ('d', 1), ('e', 1), ('f', 1),])
        )
    }
}
