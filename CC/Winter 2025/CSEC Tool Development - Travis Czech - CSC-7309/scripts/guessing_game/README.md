# Guessing Game — Rust Book Chapter 2

**Course:** CSC-7309 · **Week:** 5 · **Date:** 2025-02-05

## Overview

Implementation of the Guessing Game from [The Rust Programming Language, Chapter 2](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html). A number-guessing game that ties together concepts from Weeks 1–4: variables, mutability, input parsing, pattern matching, and external crate usage.

## Features

- Random secret number generation (1–100) using `rand` crate
- Interactive input loop with `io::stdin().read_line()`
- Graceful error handling via `match` on `Result<T, E>` (no panics on bad input)
- Attempt counter providing feedback on game completion
- Exhaustive `match` on `Ordering` enum (`Less`, `Greater`, `Equal`)

## Build & Run

```bash
cargo build
cargo run
cargo test   # 7 unit tests
```

## Key Concepts

- Variable shadowing (`let guess: u32 = guess.trim().parse()`)
- `match` on `Result` for error recovery (vs. `.expect()` which panics)
- `std::cmp::Ordering` enum with exhaustive pattern matching
- `loop { ... break }` for controlled iteration
- External crate usage (`rand::Rng`)

## Attribution

Tutorial design: The Rust Programming Language, Chapter 2. Course integration by Travis Czech (CSC-7309). Implementation, attempt counter extension, tests, and documentation by Ross Moravec.
