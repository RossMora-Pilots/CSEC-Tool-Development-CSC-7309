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
use rand::seq::IndexedRandom;

/// Default word list for the Hangman game.
const WORD_LIST: [&str; 5] = ["rust", "hangman", "programming", "cipher", "encryption"];

/// Number of incorrect guesses allowed before game over.
const DEFAULT_MAX_ATTEMPTS: u8 = 6;

/// First-pass Hangman game state using `Vec<char>` for guessed letters.
///
/// This v1 implementation uses string-based state tracking (`"won"`/`"lost"`/`"playing"`)
/// and `Vec<char>` for O(n) guess lookups. See `hangman_refined` for the improved version.
struct Hangman {
    word: Vec<char>,
    guessed: Vec<char>,
    attempts_left: u8,
}

impl Hangman {
    /// Creates a new game with a random word from the provided list.
    fn new(words: &[&str], max_attempts: u8) -> Self {
        let chosen_word = words.choose(&mut rand::rng()).unwrap();
        Hangman {
            word: chosen_word.chars().collect(),
            guessed: Vec::new(),
            attempts_left: max_attempts,
        }
    }

    /// Checks if a letter exists in the target word.
    fn contains(&self, letter: char) -> bool {
        self.word.contains(&letter)
    }

    /// Processes a letter guess, adding to guessed list and decrementing attempts if wrong.
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

    /// Prints the word with unguessed letters masked as underscores.
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

    /// Returns the current game state as a string: `"won"`, `"lost"`, or `"playing"`.
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
    let words = WORD_LIST;
    let max_attempts = DEFAULT_MAX_ATTEMPTS;
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

// ───────────────────────────────────────────────────────────
// Unit Tests
// ───────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: create a Hangman game with a known word for deterministic testing.
    fn game_with_word(word: &str, max_attempts: u8) -> Hangman {
        Hangman {
            word: word.chars().collect(),
            guessed: Vec::new(),
            attempts_left: max_attempts,
        }
    }

    #[test]
    fn new_game_starts_in_playing_state() {
        let game = game_with_word("rust", 6);
        assert_eq!(game.check_state(), "playing");
    }

    #[test]
    fn correct_guess_does_not_reduce_attempts() {
        let mut game = game_with_word("rust", 6);
        game.make_guess('r');
        assert_eq!(game.attempts_left, 6);
        assert!(game.guessed.contains(&'r'));
    }

    #[test]
    fn incorrect_guess_reduces_attempts() {
        let mut game = game_with_word("rust", 6);
        game.make_guess('z');
        assert_eq!(game.attempts_left, 5);
    }

    #[test]
    fn duplicate_guess_does_not_reduce_attempts() {
        let mut game = game_with_word("rust", 6);
        game.make_guess('z');
        assert_eq!(game.attempts_left, 5);
        game.make_guess('z'); // duplicate
        assert_eq!(game.attempts_left, 5); // unchanged
    }

    #[test]
    fn guessing_all_letters_wins() {
        let mut game = game_with_word("hi", 6);
        game.make_guess('h');
        game.make_guess('i');
        assert_eq!(game.check_state(), "won");
    }

    #[test]
    fn running_out_of_attempts_loses() {
        let mut game = game_with_word("rust", 2);
        game.make_guess('x');
        game.make_guess('y');
        assert_eq!(game.check_state(), "lost");
    }

    #[test]
    fn display_word_masks_unguessed_letters() {
        let game = game_with_word("rust", 6);
        let display: String = game.word.iter()
            .map(|&c| if game.guessed.contains(&c) { c } else { '_' })
            .collect();
        assert_eq!(display, "____");
    }

    #[test]
    fn display_word_reveals_guessed_letters() {
        let mut game = game_with_word("rust", 6);
        game.make_guess('r');
        game.make_guess('t');
        let display: String = game.word.iter()
            .map(|&c| if game.guessed.contains(&c) { c } else { '_' })
            .collect();
        assert_eq!(display, "r__t");
    }
}
