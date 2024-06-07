use colored::*;
use bracket_random::prelude::RandomNumberGenerator;
use std::collections::HashSet;
use std::io::{self, Write};

const ALL_WORDS: &str = include_str!("../data/words.txt");
const WORD_LEN: usize = 5;
const MAX_TRIES: usize = 6;

fn main() {
    let mut wordle = Wordle::new();
    while !wordle.is_game_over() {
        wordle.ask_for_guess();
        wordle.display_guesses();
    }
}

struct Wordle {
    dictionary: Vec<String>,
    word: String,
    invalid_letters: HashSet<char>,
    guesses: Vec<String>,
}

impl Wordle {
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let dictionary = ALL_WORDS
            .lines()
            .map(preprocess)
            .filter(|word| word.len() == WORD_LEN)
            .collect::<Vec<_>>();
        let word = rng.random_slice_entry(&dictionary).expect("No words found").clone();
        Self {
            dictionary,
            word,
            invalid_letters: HashSet::new(),
            guesses: Vec::new(),
        }
    }

    fn ask_for_guess(&mut self) -> String {
        println!("{}", format!("Enter your guess ({} letters):", WORD_LEN).cyan());
        self.display_invalid_letters();
        let mut guess = String::new();
        loop {
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut guess).unwrap();
            guess = preprocess(&guess);
            if guess.len() != WORD_LEN {
                println!("{}", format!("Your guess must be {} letters.", WORD_LEN).red());
            } else if !self.dictionary.contains(&guess) {
                println!("{}", format!("{} is not in the dictionary.", guess).red());
            } else {
                self.guesses.push(guess.clone());
                return guess;
            }
            guess.clear();
        }
    }

    fn display_guesses(&mut self) {
        for (index, guess) in self.guesses.iter().enumerate() {
            print!("{} ", index + 1);
            for (pos, ch) in guess.chars().enumerate() {
                let output = if self.word.chars().nth(pos) == Some(ch) {
                    ch.to_string().green()
                } else if self.word.contains(ch) {
                    ch.to_string().yellow()
                } else {
                    self.invalid_letters.insert(ch);
                    ch.to_string().red()
                };
                print!("{}", output);
            }
            println!();
        }
    }

    fn display_invalid_letters(&self) {
        if !self.invalid_letters.is_empty() {
            println!("Invalid letters: {}", self.invalid_letters.iter().collect::<String>().red());
        }
    }

    fn is_game_over(&self) -> bool {
        if let Some(last_guess) = self.guesses.last() {
            if last_guess == &self.word {
                println!("Congratulations! You guessed the word in {} tries.", self.guesses.len());
                return true;
            } else if self.guesses.len() >= MAX_TRIES {
                println!("{}", format!("Game over! The word was: {}", self.word).red());
                return true;
            }
        }
        false
    }
}

fn preprocess(word: &str) -> String {
    word.trim().to_uppercase().chars().filter(|c| c.is_ascii_alphabetic()).collect()
}
