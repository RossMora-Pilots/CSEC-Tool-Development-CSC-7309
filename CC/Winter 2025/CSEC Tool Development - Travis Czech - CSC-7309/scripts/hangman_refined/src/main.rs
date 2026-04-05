// CSC-7309 — Week 4 — Hangman (refined)
//
// Course: CSEC Tool Development, Winter 2025, Instructor Travis Czech
// Purpose: Improved version of the Week 4 Hangman exercise. Compared to
// hangman_v1, this refined implementation:
//
//   1. Uses an enum (GameState::{Playing, Won, Lost}) instead of stringly-typed
//      state tracking — type-safe and exhaustively checked by the compiler.
//   2. Uses HashSet<char> for the set of guessed letters — O(1) membership
//      checks vs. Vec<char>'s O(n).
//   3. Uses `saturating_sub(1)` to protect attempts_left from underflow.
//   4. Normalizes user input with `.to_lowercase()` for case-insensitive play.
//
// These refinements reflect typical Rust idioms and were discussed during
// the Week 4 lecture (2025-01-29).
//
// Build:  cargo build
// Run:    cargo run

// We need to bring in several things from the standard library and external crates:
//
// - std::io for reading input from stdin
// - std::collections::HashSet for storing guessed letters and checking membership quickly
// - rand::seq::SliceRandom for picking a random word from the list
//
use std::collections::HashSet;
use std::io;
use rand::seq::SliceRandom; // Make sure you have the `rand` crate in your Cargo.toml

// The GameState enum represents the different outcomes
// the game can be in at any point in time.
#[derive(Debug)]
enum GameState {
    Playing, // The player is still guessing
    Won,     // The player has guessed the word correctly
    Lost,    // The player has run out of attempts
}

// The Hangman struct holds all the information we need for the game:
//
// 1. word: the target word the player is trying to guess,
//          stored as a vector of characters for easy iteration.
// 2. guessed: a HashSet of characters the player has already guessed.
// 3. attempts_left: how many incorrect guesses the player can still make.
struct Hangman {
    word: Vec<char>,
    guessed: HashSet<char>,
    attempts_left: u8,
}

impl Hangman {
    // The new() function creates a new game:
    // - It chooses a random word from the provided &str slice
    // - It converts that word into a Vec<char> so we can easily check each character.
    // - It initializes an empty HashSet for guessed letters.
    // - It sets the number of attempts the player has.
    fn new(words: &[&str], max_attempts: u8) -> Self {
        // choose() selects one random element from the slice
        // If the slice is empty, we call .expect() to panic with a friendly message
        let chosen_word = words
            .choose(&mut rand::thread_rng())
            .expect("Words list cannot be empty.");

        // Collect each character of the chosen word into a vector
        let word_chars = chosen_word.chars().collect();

        // Construct the Hangman struct and return it
        Hangman {
            word: word_chars,
            guessed: HashSet::new(),
            attempts_left: max_attempts,
        }
    }

    // The state() function returns the current state of the game:
    // - GameState::Won if every character in `word` is contained in `guessed`
    // - GameState::Lost if attempts_left is 0
    // - Otherwise, GameState::Playing
    fn state(&self) -> GameState {
        // Check if all characters in the word have been guessed
        if self.word.iter().all(|&c| self.guessed.contains(&c)) {
            GameState::Won
        }
        // If there are no attempts left, the player loses
        else if self.attempts_left == 0 {
            GameState::Lost
        }
        // Otherwise, the game continues
        else {
            GameState::Playing
        }
    }

    // The display_word() function shows the current progress of the word to the player:
    // e.g., if the word is "hangman" and the player has guessed 'a' and 'n',
    //       it might display "_a_n_a_"
    fn display_word(&self) {
        // For each character in the `word`, if it's in `guessed`, show it;
        // otherwise, display an underscore '_'
        let display: String = self
            .word
            .iter()
            .map(|&c| if self.guessed.contains(&c) { c } else { '_' })
            .collect();

        // Print the resulting string
        println!("Word: {}", display);
    }

    // The make_guess() function takes in a single character and updates the game state:
    // - If the character has not been guessed before, add it to `guessed`.
    // - If that character isn't in `word`, decrement attempts_left.
    // - If the character was already guessed, inform the user and don't change anything.
    fn make_guess(&mut self, letter: char) {
        // Check if we've already guessed this character
        if !self.guessed.contains(&letter) {
            // If not, insert it into the HashSet of guessed characters
            self.guessed.insert(letter);

            // If the word doesn't contain this letter, we lose an attempt
            if !self.word.contains(&letter) {
                // We use saturating_sub(1) to avoid accidental underflow,
                // though in practice logic should prevent attempts_left from going below 0
                self.attempts_left = self.attempts_left.saturating_sub(1);
                println!("Incorrect guess! Attempts left: {}", self.attempts_left);
            }
        } else {
            // We already guessed this letter, so let the player know
            println!("You already guessed '{}'. Try another letter.", letter);
        }
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
            guessed: HashSet::new(),
            attempts_left: max_attempts,
        }
    }

    #[test]
    fn new_game_starts_in_playing_state() {
        let game = game_with_word("rust", 6);
        assert!(matches!(game.state(), GameState::Playing));
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
        assert!(matches!(game.state(), GameState::Won));
    }

    #[test]
    fn running_out_of_attempts_loses() {
        let mut game = game_with_word("rust", 2);
        game.make_guess('x');
        game.make_guess('y');
        assert!(matches!(game.state(), GameState::Lost));
    }

    #[test]
    fn saturating_sub_prevents_underflow() {
        let mut game = game_with_word("rust", 1);
        game.make_guess('x'); // attempts_left → 0
        assert_eq!(game.attempts_left, 0);
        // Attempting another wrong guess should not underflow
        game.make_guess('y'); // already at 0, duplicate-guarded by HashSet
        assert_eq!(game.attempts_left, 0);
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

fn main() {
    // A list of possible words for the player to guess
    let words = ["rust", "hangman", "programming", "cipher", "encryption"];

    // The maximum number of attempts (incorrect guesses) allowed
    let max_attempts = 6;

    // Create a new Hangman game using our words list and maximum attempts
    let mut game = Hangman::new(&words, max_attempts);

    println!("Welcome to Hangman!");

    // We use a loop to continuously prompt the player for guesses
    // until the game is in the Won or Lost state.
    loop {
        // First, check the current state of the game
        match game.state() {
            // If the game is still ongoing, let the player guess
            GameState::Playing => {
                // Show the player how much of the word they've guessed so far
                game.display_word();

                // Prompt the user for their next guess
                println!("Enter your guess: ");

                // We'll read a line from stdin into a String
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");

                // Trim whitespace and convert the input to lowercase for consistency
                let guess = input.trim().to_lowercase();

                // We only want to handle the first character of their input
                if let Some(letter) = guess.chars().next() {
                    // Check if the character is alphabetic (A-Z)
                    if letter.is_alphabetic() {
                        // Pass the letter to our Hangman struct to update the game
                        game.make_guess(letter);
                    } else {
                        // If it's not a letter, ask the user to try again
                        println!("Please enter a valid letter.");
                    }
                } else {
                    // If the user didn't type anything (or typed whitespace), handle that
                    println!("No input detected.");
                }
            }
            // If the player has guessed all letters correctly, they win
            GameState::Won => {
                // Show the fully guessed word
                game.display_word();
                println!("Congratulations! You guessed the word.");
                break; // Exit the loop and end the program
            }
            // If the player has no attempts left, they lose
            GameState::Lost => {
                // Reveal the word so the player knows what it was
                let correct_word: String = game.word.iter().collect();
                println!("Game over! The word was: {}", correct_word);
                break; // Exit the loop
            }
        }
    }
}
