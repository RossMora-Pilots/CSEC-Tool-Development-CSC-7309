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

## Attribution

Tutorial design: The Rust Programming Language (official book), Chapter 2. Course integration by Travis Czech (CSC-7309, Week 5). Student implementation and writeup by Ross Moravec.
