# Student-Authored Rust Scripts — CSC-7309

This folder contains **student-authored Rust code** from the Winter 2025 CSEC Tool Development course. All programs are standalone Cargo projects and can be built/run independently.

> [!TIP]
> Run `cargo test` in `hangman_refined/` to execute 9 unit tests validating the game's state machine.

## Projects

### 1. `hangman_v1/` — First-Pass Hangman (Week 4)

The initial live-coded version from the 2025-01-29 lecture. Demonstrates:

- Defining a `struct Hangman` with `word`, `guessed`, and `attempts_left` fields
- Implementing methods via `impl Hangman { ... }`
- Iterating over a `Vec<char>` with `.iter()` and closures
- Random word selection with the `rand` crate

Build & run:

```bash
cd hangman_v1
cargo build
cargo run
```

**Observations during class:** Stringly-typed state (`"playing"`, `"won"`, `"lost"`) works but is fragile — a typo isn't caught by the compiler. Using `Vec<char>` for guessed letters is O(n) per lookup.

### 2. `hangman_refined/` — Improved Hangman (Week 4 wrap-up)

A refactor incorporating Rust best practices introduced later in the lecture:

| Concern | `hangman_v1` | `hangman_refined` |
|---|---|---|
| State representation | `String` ("playing"/"won"/"lost") | `enum GameState { Playing, Won, Lost }` |
| Guessed letters | `Vec<char>` (O(n) contains) | `HashSet<char>` (O(1) contains) |
| Attempts underflow | `self.attempts_left -= 1` (panics on underflow) | `self.attempts_left.saturating_sub(1)` |
| Input normalization | First char only, no case fold | `.trim().to_lowercase()` |
| Control flow | `while game.check_state() == "playing"` | `loop { match game.state() { ... } }` |
| Unit tests | None | 9 tests (`cargo test`) |

Build & run:

```bash
cd hangman_refined
cargo build
cargo run
cargo test    # 9 unit tests
```

### 3. `guessing_game/` — Guessing Game (Week 5)

Implementation of the [Rust Book Chapter 2 tutorial](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html). Demonstrates:

- Reading from stdin with `io::stdin().read_line()`
- Graceful error handling with `match` on `Result<T, E>` (no panics)
- Pattern matching with `Ordering::{Less, Greater, Equal}`
- Loop control with `loop { ... break }`
- Variable shadowing (reusing `guess` name after parsing)

Build & run:

```bash
cd guessing_game
cargo build
cargo run
```

## Original Source Files

The original class-provided source files (`Hangman_v1.txt` and `Refined Hangman with comments.txt`) remain in their original location at `D:\CC\Winter2025\...\Week 4\` for archival. The `.rs` files here are faithful adaptations with Cargo project structure, proper formatting, and contextual comments added for portfolio clarity.

## Verification

All three projects should compile cleanly on a system with:

- Rust 1.70+ (stable, via Rustup)
- `cargo --version` reporting 1.70 or newer
- Internet access for the `rand` crate's first build

Run `cargo check` in each directory to validate without producing a binary.
