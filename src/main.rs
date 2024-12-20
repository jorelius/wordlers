use bracket_random::prelude::RandomNumberGenerator;
use colored::*;
use std::collections::HashSet;

const WORD_LENGTH: usize = 5;
const MAX_TRIES: usize = 6;
const ALL_WORDS: &str = include_str!("words.txt");

struct WordlerGame {
    dictionary: Vec<String>,
    word: String,
    guessed_letters: HashSet<char>,
    guesses: Vec<String>,
}

impl WordlerGame {
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let dictionary = words_list();
        let word = rng.random_slice_entry(&dictionary).unwrap().clone();
        Self {
            dictionary,
            word,
            guessed_letters: HashSet::new(),
            guesses: Vec::new(),
        }
    }

    /**
     * Display user guesses
     */
    fn display_guesses(&mut self) {
        self.guesses
            .iter()
            .enumerate()
            .for_each(|(guess_number, guess)| {
                print!("{}: ", guess_number + 1);
                guess.chars().enumerate().for_each(|(pos, c)| {
                    let display = if self.word.chars().nth(pos).unwrap() == c {
                        format!("{c}").bright_green()
                    } else if self.word.chars().any(|wc| wc == c) {
                        format!("{c}").bright_yellow()
                    } else {
                        self.guessed_letters.insert(c);
                        format!("{c}").red()
                    };
                    print!("{display}");
                });
                println!();
            })
    }

    /**
     * Display guess letter validation
     */
    fn display_invalid_letters(&self) {
        if !self.guessed_letters.is_empty() {
            print!("Letters not in the word: ");
            self.guessed_letters
                .iter()
                .for_each(|letter| print!("{letter} "));
            println!();
        }
    }

    /**
     * Ask User for a guess
     */
    fn ask_for_guess(&mut self) -> String {
        println!(
            "{}",
            format!(
                "Enter your word guess ({} letters) and press ENTER",
                WORD_LENGTH
            )
            .cyan()
        );
        self.display_invalid_letters();
        let mut guess = String::new();
        let mut valid_guess = false;
        while !valid_guess {
            guess = String::new();
            std::io::stdin().read_line(&mut guess).unwrap();
            guess = sanitize_word(&guess);
            if guess.len() != WORD_LENGTH {
                println!(
                    "{}",
                    format!("Your guess must be {} letters.", WORD_LENGTH).red()
                )
            } else if !self.dictionary.iter().any(|word| word == &guess) {
                println!("{}", "{guess} isn't in the Rustle dictionary.".red())
            } else {
                self.guesses.push(guess.clone());
                valid_guess = true;
            }
        }
        guess
    }

    /**
     * Display end of game status
     */
    fn is_game_over(&self, guess: &str) -> bool {
        let n_tries = self.guesses.len();
        if guess == self.word {
            println!("Correct! You guessed the word in {} tries.", n_tries);
            true
        } else if n_tries >= MAX_TRIES {
            println!(
                "{}",
                format!("You ran out of tries! The word was {}", self.word).bright_red()
            );
            true
        } else {
            false
        }
    }
}

/**
 * Main Game loop
 */
fn main() {
    let mut game = WordlerGame::new();
    loop {
        game.display_guesses();
        let guess = game.ask_for_guess();
        if game.is_game_over(&guess) {
            break;
        }
    }
}

/**
 * Extract word list from all words string
 */
fn words_list() -> Vec<String> {
    ALL_WORDS
        .split('\n')
        .skip(2)
        .map(sanitize_word)
        .filter(|line| line.len() == 5)
        .collect()
}

/**
 * Sanitize string (uppercase, alphabetical characters)
 */
fn sanitize_word(word: &str) -> String {
    word.trim()
        .to_uppercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect()
}
