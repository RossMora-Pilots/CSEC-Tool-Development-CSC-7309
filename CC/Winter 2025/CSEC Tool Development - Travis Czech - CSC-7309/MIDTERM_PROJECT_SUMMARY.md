# Midterm Project Summary — Hangman in Rust

**Course:** CSEC Tool Development (CSC-7309) | **Week:** 4 | **Date:** 2025-01-29 | **Instructor:** Travis Czech

The Week 4 Hangman game is the largest integrated program written during the first half of the course. It exercises every concept introduced in Weeks 1–4 (variables, mutability, primitive types, ownership, borrowing, references, structs, methods, enums, and pattern matching) in a single working terminal application. Two versions were produced: a first-pass version written live in lecture (`hangman_v1`), and a refined version demonstrating idiomatic Rust patterns (`hangman_refined`).

---

## Project Overview

Build a single-player terminal Hangman game:

- Pick a random word from a fixed word list
- Reveal the word character-by-character as the player guesses letters correctly
- Limit the player to a fixed number of incorrect attempts (default: 6)
- Report win / loss and reveal the final word

## Objectives Achieved

| # | Objective | Evidence |
|---|---|---|
| 1 | Define a custom data type with `struct` | `struct Hangman { word, guessed, attempts_left }` |
| 2 | Implement behavior via `impl` blocks | `Hangman::new`, `state`, `display_word`, `make_guess` |
| 3 | Use an associated function as a constructor | `Hangman::new(&words, max_attempts)` |
| 4 | Use an `enum` to represent mutually exclusive states | `enum GameState { Playing, Won, Lost }` |
| 5 | Use `match` for exhaustive state handling | `match game.state() { … }` |
| 6 | Demonstrate ownership & borrowing | `&[&str]`, `&self`, `&mut self` |
| 7 | Use a third-party crate | `rand::seq::SliceRandom::choose` |
| 8 | Use a standard-library collection | `HashSet<char>` (refined) / `Vec<char>` (v1) |
| 9 | Handle user input robustly | `io::stdin().read_line()` + trim + lowercase |
| 10 | Prevent panics with safe arithmetic | `.saturating_sub(1)` (refined) |

## Architecture

### Program Flow

```mermaid
graph TD
    M[main] --> W["words: [&str; 5]"]
    W --> N["Hangman::new(&words, 6)"]
    N --> L["loop { match game.state() }"]

    L --> P{GameState::Playing}
    L --> WN{GameState::Won}
    L --> LS{GameState::Lost}

    P --> D1[display_word]
    D1 --> R[read stdin]
    R --> G["make_guess(&mut self, char)"]
    G --> L

    WN --> D2[display_word]
    D2 --> C["Congratulations! break"]

    LS --> RV["Reveal word. break"]

    style P fill:#2b6cb0,color:#fff
    style WN fill:#38a169,color:#fff
    style LS fill:#e53e3e,color:#fff
```

### State Machine

```mermaid
stateDiagram-v2
    [*] --> Playing: Hangman::new()
    Playing --> Playing: Correct guess (letter in word)
    Playing --> Playing: Incorrect guess (attempts > 0)
    Playing --> Won: All letters guessed
    Playing --> Lost: attempts_left == 0
    Won --> [*]: 🎉 Congratulations
    Lost --> [*]: 💀 Game Over
```

### Data Flow

```
user char → make_guess(&mut self, c) → guessed: HashSet<char>
                                        ↓ contains check
                                     word: Vec<char>
                                        ↓ subtraction
                                     attempts_left: u8
                                        ↓ transition
                                     GameState (enum)
```

## Key Implementations

### 1. Random Word Selection

```rust
let chosen_word = words
    .choose(&mut rand::thread_rng())
    .expect("Words list cannot be empty.");
```

Uses the `rand` crate's `SliceRandom` trait. Returns `Option<&T>` — we `.expect()` with a message because an empty word list would be a programming error, not a runtime error the user could recover from.

### 2. Guess Processing (refined version)

```rust
fn make_guess(&mut self, letter: char) {
    if !self.guessed.contains(&letter) {
        self.guessed.insert(letter);
        if !self.word.contains(&letter) {
            self.attempts_left = self.attempts_left.saturating_sub(1);
            println!("Incorrect guess! Attempts left: {}", self.attempts_left);
        }
    } else {
        println!("You already guessed '{}'. Try another letter.", letter);
    }
}
```

### 3. Word Display with Masking

```rust
fn display_word(&self) {
    let display: String = self.word.iter()
        .map(|&c| if self.guessed.contains(&c) { c } else { '_' })
        .collect();
    println!("Word: {}", display);
}
```

### 4. State Check

```rust
fn state(&self) -> GameState {
    if self.word.iter().all(|&c| self.guessed.contains(&c)) {
        GameState::Won
    } else if self.attempts_left == 0 {
        GameState::Lost
    } else {
        GameState::Playing
    }
}
```

## v1 → Refined Changes (Improvements)

```mermaid
graph LR
    subgraph "v1 (First Pass)"
        A1["State: String<br/>'playing' / 'won' / 'lost'"]
        A2["Guesses: Vec&lt;char&gt;<br/>O(n) lookup"]
        A3["Underflow: -= 1<br/>⚠️ Can panic"]
        A4["Input: First char only"]
        A5["Loop: while + string compare"]
    end

    subgraph "Refined (Idiomatic)"
        B1["State: enum GameState<br/>Compiler-checked"]
        B2["Guesses: HashSet&lt;char&gt;<br/>O(1) lookup"]
        B3["Underflow: saturating_sub<br/>✅ Safe"]
        B4["Input: .to_lowercase()"]
        B5["Loop: loop + match"]
    end

    A1 -->|"Type Safety"| B1
    A2 -->|"Performance"| B2
    A3 -->|"Safety"| B3
    A4 -->|"UX"| B4
    A5 -->|"Idiomatic"| B5

    style A1 fill:#e53e3e,color:#fff
    style A3 fill:#e53e3e,color:#fff
    style B1 fill:#38a169,color:#fff
    style B2 fill:#38a169,color:#fff
    style B3 fill:#38a169,color:#fff
```

| Aspect | v1 | Refined | Reason |
|---|---|---|---|
| State | `String` | `enum GameState` | Type safety, compiler-checked |
| Guesses | `Vec<char>` | `HashSet<char>` | O(1) lookup vs. O(n) |
| Underflow | `x -= 1` | `.saturating_sub(1)` | No panic possible |
| Case | First char only | `.to_lowercase()` | User experience |
| Loop | `while` + `"playing"` | `loop` + `match` | Idiomatic Rust |
| Tests | None | 9 unit tests | Validates state transitions |

## Metrics

| Metric | v1 | Refined |
|---|---|---|
| Source lines (excl. comments/blanks) | ~75 | ~95 |
| Total source lines | 92 | 181 |
| Struct fields | 3 | 3 |
| Public methods | 5 | 4 |
| External crates | 1 (`rand`) | 1 (`rand`) |
| Stdlib collections | `Vec` | `Vec`, `HashSet` |
| Compiler warnings (target) | 0 | 0 |

## Evidence Links

- [scripts/hangman_v1/src/main.rs](scripts/hangman_v1/src/main.rs) — First-pass source
- [scripts/hangman_refined/src/main.rs](scripts/hangman_refined/src/main.rs) — Refined source
- [scripts/hangman_v1/Cargo.toml](scripts/hangman_v1/Cargo.toml) — Package manifest (v1)
- [scripts/hangman_refined/Cargo.toml](scripts/hangman_refined/Cargo.toml) — Package manifest (refined)
- [WEEKS_1-6_RUST_FUNDAMENTALS_SUMMARY.md](WEEKS_1-6_RUST_FUNDAMENTALS_SUMMARY.md) — Underpinning concepts

## Retrospective / Learning Reflection

**What worked:**

- Writing v1 first, then refactoring to idiomatic Rust, made the language's design choices much clearer. The compiler gently guided me from "works" to "idiomatic."
- `HashSet` was an obvious win once I saw `Vec::contains` is O(n).
- `enum GameState` made the control flow self-documenting.

**What was hard:**

- The borrow checker initially felt frustrating. Understanding *why* Rust forbids certain patterns (to prevent data races and use-after-free) made it less adversarial.
- Deciding when to `.clone()` vs. pass `&self` required a clearer mental model of heap allocations.

**What I'd do next:**

- Add unit tests for `make_guess` and `state` transitions
- Extract the word list to a file or CLI argument
- Add a text-based "gallows" visualization per wrong guess
- Internationalize the word list

## Attribution

Algorithm and initial live-coded implementation © Travis Czech (CSC-7309 lecture, 2025-01-29). Student refactoring, Cargo project structure, documentation, and analysis by Ross Moravec.
