// CSC-7309 — Week 4 — Hangman v1 (first pass)
//
// Course: CSEC Tool Development, Winter 2025, Instructor Travis Czech
// Purpose: Demonstrate Rust structs, methods (impl), and basic game-state
// tracking with Vec<char> and string-based state signaling.
//
// This is the FIRST pass produced during live coding in class. A refined
// version (see ../hangman_refined) improves state tracking with an enum
// and uses HashSet<char> for O(1) lookup of guessed letters.
//
// Build:  cargo build
// Run:    cargo run

use std::io;
use rand::seq::SliceRandom;

// Define the Hangman struct
struct Hangman {
    word: Vec<char>,
    guessed: Vec<char>,
    attempts_left: u8,
}

impl Hangman {
    // Create a new game with a random word
    fn new(words: &[&str], max_attempts: u8) -> Self {
        let chosen_word = words.choose(&mut rand::thread_rng()).unwrap();
        Hangman {
            word: chosen_word.chars().collect(),
            guessed: Vec::new(),
            attempts_left: max_attempts,
        }
    }

    // Check if a letter is in the word
    fn contains(&self, letter: char) -> bool {
        self.word.contains(&letter)
    }

    // Update game state with a guess
    fn make_guess(&mut self, letter: char) {
        if !self.guessed.contains(&letter) {
            self.guessed.push(letter);
            if !self.contains(letter) {
                self.attempts_left -= 1;
                println!("Incorrect guess! Attempts left: {}", self.attempts_left);
            }
        } else {
            println!("You already guessed '{}'. Try another letter.", letter);
        }
    }

    // Display the current progress of the word
    fn display_word(&self) {
        let display: String = self.word.iter().map(|&c| {
            if self.guessed.contains(&c) {
                c
            } else {
                '_'
            }
        }).collect();
        println!("Word: {}", display);
    }

    // Check if the player has won or lost
    fn check_state(&self) -> String {
        if self.word.iter().all(|&c| self.guessed.contains(&c)) {
            "won".to_string()
        } else if self.attempts_left == 0 {
            "lost".to_string()
        } else {
            "playing".to_string()
        }
    }
}

fn main() {
    let words = ["rust", "hangman", "programming", "cipher", "encryption"];
    let max_attempts = 6;
    let mut game = Hangman::new(&words, max_attempts);

    println!("Welcome to Hangman!");

    while game.check_state() == "playing" {
        game.display_word();
        println!("Enter your guess: ");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let letter = input.trim().chars().next().unwrap_or(' ');

        if letter.is_alphabetic() {
            game.make_guess(letter);
        } else {
            println!("Please enter a valid letter.");
        }
    }

    game.display_word();
    match game.check_state().as_str() {
        "won" => println!("Congratulations! You guessed the word."),
        "lost" => println!("Game over! The word was: {:?}", game.word.iter().collect::<String>()),
        _ => {}
    }
}
