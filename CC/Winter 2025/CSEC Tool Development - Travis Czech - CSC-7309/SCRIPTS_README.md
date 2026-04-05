# Scripts — Usage and Notes

This document summarizes the purpose and basic usage of each Rust source file and Cargo project in this course portfolio.

## Student-Authored Cargo Projects (scripts/)

### `hangman_v1/` — First-Pass Hangman (Week 4) — [✅ OK]

- **Entry point:** `src/main.rs`
- **Lines of code:** 92 (incl. comments)
- **Dependencies:** `rand = "0.9"`
- **Concepts demonstrated:** `struct`, `impl`, associated functions, methods, `Vec<char>`, `SliceRandom::choose`
- **Build & run:**

  ```bash
  cd scripts/hangman_v1
  cargo run
  ```

- **Classroom origin:** Live-coded by instructor Travis Czech on 2025-01-29. Stored originally as `Hangman_v1.txt` at `D:\CC\...\Week 4\`.

### `hangman_refined/` — Refined Hangman (Week 4) — [✅ OK]

- **Entry point:** `src/main.rs`
- **Lines of code:** 181 (incl. extensive comments) + 80 (unit tests)
- **Dependencies:** `rand = "0.9"`
- **Concepts demonstrated:** `enum GameState`, `HashSet<char>`, `match`, `saturating_sub`, `.to_lowercase()`, `#[cfg(test)]`
- **Unit tests:** 9 tests covering state transitions, guess processing, display masking
- **Build & run:**

  ```bash
  cd scripts/hangman_refined
  cargo run
  cargo test   # runs 9 unit tests
  ```

- **Classroom origin:** Instructor refactor of the first-pass version. Stored originally as `Refined Hangman with comments.txt` at `D:\CC\...\Week 4\`.

### `guessing_game/` — Guessing Game (Week 5) — [✅ OK]

- **Entry point:** `src/main.rs`
- **Lines of code:** ~70 (incl. comments)
- **Dependencies:** `rand = "0.9"`
- **Concepts demonstrated:** `io::stdin`, `match` on `Result` and `Ordering`, `loop`/`break`, variable shadowing, `.trim().parse()`
- **Build & run:**

  ```bash
  cd scripts/guessing_game
  cargo run
  ```

- **Classroom origin:** Rust Book Chapter 2 tutorial, completed during Week 5 lab session (2025-02-05). See [assignments/Assignment02_GuessingGame.md](assignments/Assignment02_GuessingGame.md).

## Provided/External Scripts (scripts-extra/)

None currently. See [scripts-extra/README.md](scripts-extra/README.md) for external-reference links (Rust Book, rand crate docs, instructor-shared URLs from Week 1 chat).

---

## Safety & Responsible Use

> [!CAUTION]
> All scripts here are **educational reference code**. They are benign programs demonstrating Rust fundamentals. This portfolio includes discussion of security-tool patterns (keyloggers, scanners). **No working offensive tooling is published.** If adapting these examples to security-tool code, always:
>
> 1. Work inside a VM with revertible snapshots.
> 2. Test only against systems you own or have explicit written authorization to test.
> 3. Honor applicable laws (Canadian Criminal Code s. 342.1, CFAA, etc.) and institutional AUPs.

## Verification Checklist

Before committing changes to any script:

- [ ] `cargo check` succeeds
- [ ] `cargo build` succeeds
- [ ] `cargo run` produces the expected output
- [ ] No hard-coded secrets or credentials
- [ ] No PII (names outside the instructor's public attribution)
- [ ] Comments reference the course (CSC-7309) and instructor
