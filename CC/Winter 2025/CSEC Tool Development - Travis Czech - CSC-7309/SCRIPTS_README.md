# Scripts — Usage and Notes

This document summarizes the purpose and basic usage of each Rust source file and Cargo project in this course portfolio.

## Student-Authored Cargo Projects (scripts/)

### `hangman_v1/` — First-Pass Hangman (Week 4) — [✅ OK]

- **Entry point:** `src/main.rs`
- **Lines of code:** 92 (incl. comments)
- **Dependencies:** `rand = "0.8"`
- **Concepts demonstrated:** `struct`, `impl`, associated functions, methods, `Vec<char>`, `SliceRandom::choose`
- **Build & run:**

  ```bash
  cd scripts/hangman_v1
  cargo run
  ```

- **Classroom origin:** Live-coded by instructor Travis Czech on 2025-01-29. Stored originally as `Hangman_v1.txt` at `D:\CC\...\Week 4\`.

### `hangman_refined/` — Refined Hangman (Week 4) — [✅ OK]

- **Entry point:** `src/main.rs`
- **Lines of code:** 181 (incl. extensive comments)
- **Dependencies:** `rand = "0.8"`
- **Concepts demonstrated:** `enum GameState`, `HashSet<char>`, `match`, `saturating_sub`, `.to_lowercase()`
- **Build & run:**

  ```bash
  cd scripts/hangman_refined
  cargo run
  ```

- **Classroom origin:** Instructor refactor of the first-pass version. Stored originally as `Refined Hangman with comments.txt` at `D:\CC\...\Week 4\`.

## Provided/External Scripts (scripts-extra/)

None currently. See [scripts-extra/README.md](scripts-extra/README.md) for external-reference links (Rust Book, rand crate docs).

---

## Safety & Responsible Use

- All scripts here are **educational reference code**. They are benign games demonstrating Rust fundamentals.
- This portfolio includes discussion of security-tool patterns (keyloggers, scanners). **No working offensive tooling is published.**
- If adapting these examples to security-tool code, always:
  1. Work inside a VM with revertible snapshots.
  2. Test only against systems you own or have explicit written authorization to test.
  3. Honor applicable laws (Canadian Criminal Code s. 342.1, CFAA, etc.) and institutional AUPs.

## Verification Checklist

Before committing changes to any script:

- [ ] `cargo check` succeeds
- [ ] `cargo build` succeeds
- [ ] `cargo run` produces the expected output
- [ ] No hard-coded secrets or credentials
- [ ] No PII (names outside the instructor's public attribution)
- [ ] Comments reference the course (CSC-7309) and instructor
