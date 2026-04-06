//! # Guessing Game — CSC-7309 Week 5
//!
//! A number-guessing game implementing Chapter 2 of The Rust Programming Language.
//! Demonstrates `stdin` input, `match` on `Result<T, E>` and `Ordering`, loop control,
//! and variable shadowing.
//!
//! ## Course Context
//!
//! Course: CSEC Tool Development, Winter 2025, Instructor Travis Czech
//! Purpose: Reinforces concepts from Weeks 1–4:
//!
//! - Variables and mutability (let, let mut)
//! - Type conversion and shadowing
//! - Error handling with Result<T, E> and match
//! - Looping with loop { ... break }
//! - External crate usage (rand)
//! - Standard I/O (std::io::stdin)
//! - Ordering enum and pattern matching
//!
//! Reference: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
//!
//! Build:  cargo build
//! Run:    cargo run

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("=== Guessing Game ===");
    println!("I'm thinking of a number between 1 and 100.");

    // Generate a random number between 1 and 100 (inclusive)
    let secret_number = rand::rng().random_range(1..=100);

    // Track the number of attempts for feedback
    let mut attempts: u32 = 0;

    loop {
        println!("\nPlease input your guess:");

        // Create a new mutable String to hold user input
        let mut guess = String::new();

        // Read a line from stdin into the guess String
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Shadow the `guess` variable: parse the String into a u32.
        // Instead of crashing on invalid input, use match on the Result
        // to handle errors gracefully and ask again.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        attempts += 1;

        println!("You guessed: {guess}");

        // Compare the guess to the secret number using the Ordering enum.
        // match ensures we handle all three variants exhaustively.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! 🎉 It took you {attempts} attempts.");
                break;
            }
        }
    }
}

/// Evaluate a guess against the secret number.
/// Returns a human-readable hint string.
fn evaluate_guess(guess: u32, secret: u32) -> &'static str {
    match guess.cmp(&secret) {
        Ordering::Less => "Too small!",
        Ordering::Greater => "Too big!",
        Ordering::Equal => "You win!",
    }
}

// ───────────────────────────────────────────────────────────
// Unit Tests
// ───────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guess_too_small() {
        assert_eq!(evaluate_guess(10, 50), "Too small!");
    }

    #[test]
    fn guess_too_big() {
        assert_eq!(evaluate_guess(90, 50), "Too big!");
    }

    #[test]
    fn guess_correct() {
        assert_eq!(evaluate_guess(50, 50), "You win!");
    }

    #[test]
    fn boundary_guess_one() {
        assert_eq!(evaluate_guess(1, 1), "You win!");
    }

    #[test]
    fn boundary_guess_hundred() {
        assert_eq!(evaluate_guess(100, 100), "You win!");
    }

    #[test]
    fn parse_valid_input() {
        let input = "42\n";
        let result: Result<u32, _> = input.trim().parse();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn parse_invalid_input() {
        let input = "abc\n";
        let result: Result<u32, _> = input.trim().parse();
        assert!(result.is_err());
    }
}
