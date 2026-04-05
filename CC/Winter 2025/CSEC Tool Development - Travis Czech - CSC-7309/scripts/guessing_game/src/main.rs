// CSC-7309 — Week 5 — Guessing Game
//
// Course: CSEC Tool Development, Winter 2025, Instructor Travis Czech
// Purpose: Implementation of the Guessing Game from The Rust Programming
// Language (Chapter 2). Reinforces concepts from Weeks 1–4:
//
//   - Variables and mutability (let, let mut)
//   - Type conversion and shadowing
//   - Error handling with Result<T, E> and match
//   - Looping with loop { ... break }
//   - External crate usage (rand)
//   - Standard I/O (std::io::stdin)
//   - Ordering enum and pattern matching
//
// Reference: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
//
// Build:  cargo build
// Run:    cargo run

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
