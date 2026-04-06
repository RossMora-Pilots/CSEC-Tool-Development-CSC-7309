# Assignment 2 — Guessing Game (Week 5)

**Course:** CSEC Tool Development (CSC-7309) | **Week:** 5 | **Date:** 2025-02-05 | **Instructor:** Travis Czech

---

## Assignment Description

Students worked through **Chapter 2 of The Rust Programming Language** ("Programming a Guessing Game") as a guided tutorial. This exercise builds a complete interactive program that ties together concepts from Weeks 1–4.

> [!NOTE]
> The full tutorial is available at: <https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html>

## Implementation

The student-authored implementation is available as a Cargo project:

📂 **Source:** [`../scripts/guessing_game/`](../scripts/guessing_game/)

```bash
cd scripts/guessing_game
cargo run
```

## Concepts Reinforced

| Concept | How It Appears | First Learned |
|---|---|---|
| `let mut` | Mutable `guess` String and `attempts` counter | Week 2 |
| `String::new()` | Creating an empty string to hold user input | Week 2 |
| `io::stdin().read_line()` | Reading input from the terminal | Week 4 |
| `match` on `Result<T, E>` | Handling parse errors gracefully (`Ok` / `Err`) | Week 4 |
| `.trim().parse()` | Converting String → u32 with whitespace cleanup | Week 2 |
| `Ordering` enum | `Less`, `Greater`, `Equal` for numeric comparison | Week 4 |
| `loop { ... break }` | Repeating until the correct guess | Week 4 |
| Variable shadowing | Reusing `guess` name for the parsed u32 | Week 2 |
| `rand::Rng` | Generating a random secret number | Week 4 |

## Key Code Patterns

### Graceful Error Handling (No Panic)

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => {
        println!("Please type a number!");
        continue;  // skip to next loop iteration
    }
};
```

This pattern avoids `.expect()` (which panics) and instead uses `match` on the `Result` to handle invalid input gracefully — an improvement over the simpler patterns used in Week 4's Hangman.

### Exhaustive Pattern Matching

```rust
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => {
        println!("You win!");
        break;
    }
}
```

The compiler ensures all three `Ordering` variants are handled — missing one would be a compile error.

## Security Relevance

While a guessing game is not a security tool, the patterns it teaches directly apply:

- **Input validation** — Never trust user input; always parse and validate
- **Error handling without panics** — Security tools must not crash on bad input
- **Controlled loops** — Avoid infinite loops; always have exit conditions

---

## My Implementation & Challenges

### What I Built

My implementation follows the Rust Book tutorial but includes an **attempt counter** that the tutorial doesn't add — I wanted to give the player feedback on how many guesses they used:

```rust
let mut attempts: u32 = 0;
// ... inside the loop:
attempts += 1;
println!("You guessed: {guess}");
// ... on win:
println!("You win! 🎉 It took you {attempts} attempts.");
```

### Debugging Story: Shadowing Confusion

The trickiest moment was variable **shadowing**. The tutorial reuses the name `guess` for both the String input and the parsed `u32`:

```rust
let mut guess = String::new();           // guess is a String
io::stdin().read_line(&mut guess)?;
let guess: u32 = guess.trim().parse()?;  // guess is now a u32!
```

I initially thought this would cause a "variable already declared" error (coming from other languages). Instead, Rust allows **shadowing** — the second `let guess` creates a new binding that hides the first. This felt strange at first, but I realized it avoids awkward names like `guess_str` and `guess_num` while keeping the type system happy.

### Demo Output

```text
$ cargo run
   Compiling guessing_game v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.45s
     Running `target\debug\guessing_game.exe`
=== Guessing Game ===
I'm thinking of a number between 1 and 100.

Please input your guess:
50
You guessed: 50
Too big!

Please input your guess:
25
You guessed: 25
Too small!

Please input your guess:
37
You guessed: 37
Too small!

Please input your guess:
42
You guessed: 42
You win! 🎉 It took you 4 attempts.
```

### Time Spent

Approximately 1 hour for the initial tutorial walkthrough, plus 30 minutes to add the attempt counter and write unit tests.

## Attribution

Tutorial design: The Rust Programming Language (official book), Chapter 2. Course integration by Travis Czech (CSC-7309, Week 5). Student implementation and writeup by Ross Moravec.

## Competencies Achieved

- [x] Read user input from stdin with `io::stdin().read_line()`
- [x] Handle parse errors gracefully with `match` on `Result<T, E>` (no panics)
- [x] Use exhaustive pattern matching on `Ordering` enum
- [x] Implement loop control flow (`loop { ... break }`)
- [x] Apply variable shadowing for type conversion
- [x] Integrate an external crate (`rand`) via Cargo
- [x] Extend a tutorial with custom features (attempt counter)
- [x] Connect input validation patterns to security tool requirements
