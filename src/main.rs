use std::{fs::File, process::exit};

use clap::{CommandFactory, Parser};

use crate::{dictionary::Dictionary, util::validate_input};

pub mod dictionary;
pub mod repl;
pub mod trie;
pub mod util;

/// Wordle helper CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Correct letters
    #[arg(short, long, default_value = "******")]
    correct: String,

    /// Misplaced letters
    #[arg(short, long, default_value = "*****")]
    misplaced: String,

    /// Incorrect letters
    #[arg(short, long, default_value = "")]
    incorrect: String,

    /// Dictionary path
    #[arg(short, long)]
    dictionary: String,

    /// REPL mode
    #[arg(short, long)]
    repl: bool,
}

fn main() {
    let args = Args::parse();
    let dict_file = File::open(args.dictionary).unwrap();
    let mut dictionary = Dictionary::read_from_file(dict_file).unwrap();
    if args.repl {
        repl::repl(&mut dictionary);
        return;
    }
    let valid_input = [
        validate_input(&args.correct, "correct"),
        validate_input(&args.misplaced, "misplaced"),
    ]
    .iter()
    .all(|res| match res {
        Err(why) => {
            eprintln!("Validation failed for input: {why}");
            false
        }
        Ok(()) => true,
    });
    if !valid_input {
        Args::command().print_help().unwrap();
        exit(1);
    }
    dictionary.filter_words(
        args.correct.as_str(),
        args.misplaced.as_str(),
        args.incorrect.as_str(),
        &Default::default(),
    );
    let possible_words = dictionary.words();
    for word in possible_words {
        println!("{word}");
    }
}
