# Hangman Refined — Idiomatic Rust Implementation

**Course:** CSC-7309 · **Week:** 4 · **Date:** 2025-01-29

## Overview

Improved version of the Week 4 Hangman game. Refactored from `hangman_v1` to demonstrate idiomatic Rust patterns: `enum`-based state tracking, `HashSet<char>` for O(1) lookups, and safe arithmetic with `saturating_sub()`.

## Improvements Over v1

| Aspect | v1 | Refined |
|---|---|---|
| State tracking | `String` (`"won"`) | `enum GameState` (compiler-checked) |
| Guess storage | `Vec<char>` O(n) | `HashSet<char>` O(1) |
| Underflow safety | `x -= 1` (can panic) | `.saturating_sub(1)` (safe) |
| Input handling | First char only | `.to_lowercase()` for case-insensitive play |
| Testing | None | 9 unit tests |

## Build & Run

```bash
cargo build
cargo run
cargo test   # 9 unit tests
```

## Key Concepts

- `enum GameState { Playing, Won, Lost }` with exhaustive `match`
- `struct Hangman` with `HashSet<char>` for guessed letters
- `#[cfg(test)] mod tests` with deterministic test helper
- `saturating_sub(1)` to prevent unsigned integer underflow

## Attribution

Instructor refactor of v1 by Travis Czech (CSC-7309, 2025-01-29). Unit tests, Cargo project structure, and documentation by Ross Moravec.
