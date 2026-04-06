# Hangman v1 — First-Pass Implementation

**Course:** CSC-7309 · **Week:** 4 · **Date:** 2025-01-29

## Overview

First-pass Hangman game written live during the Week 4 lecture. Demonstrates Rust `struct`, `impl` blocks, associated functions, and basic game-state tracking.

## Design Decisions

- **State tracking:** `String` values (`"won"`, `"lost"`, `"playing"`) — functional but not type-safe
- **Guessed letters:** `Vec<char>` with `O(n)` `.contains()` lookups
- **Attempt decrement:** Direct subtraction (`-= 1`) — risks underflow

These design choices were intentional for a first-pass implementation. See `../hangman_refined/` for the idiomatic improvements.

## Build & Run

```bash
cargo build
cargo run
cargo test   # 8 unit tests
```

## Key Concepts

- `struct Hangman` with 3 fields (word, guessed, attempts_left)
- `impl` block with 5 methods
- `rand::seq::IndexedRandom::choose` for random word selection
- `io::stdin().read_line()` for user input

## Attribution

Live-coded by instructor Travis Czech (CSC-7309, 2025-01-29). Cargo project structure and tests by Ross Moravec.
