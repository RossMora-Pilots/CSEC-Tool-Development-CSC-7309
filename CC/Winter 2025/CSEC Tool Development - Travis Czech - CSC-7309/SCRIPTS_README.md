# Scripts — Usage and Notes

This document summarizes the purpose and basic usage of each Rust source file and Cargo project in this course portfolio.

## System Requirements

| Requirement | Minimum | Recommended |
|---|---|---|
| **Rust Toolchain** | 1.75.0+ | Latest stable via `rustup update` |
| **Cargo** | Bundled with Rust | — |
| **OS** | Linux, macOS, Windows 10+ | Any with Rust support |
| **Disk Space** | ~500 MB (toolchain + build cache) | — |
| **Internet** | Required for initial `cargo build` (downloads `rand` crate) | — |

```bash
# Verify your installation
rustc --version    # expect: rustc 1.75.0 or later
cargo --version    # expect: cargo 1.75.0 or later
```

## Student-Authored Cargo Projects (scripts/)

### `hangman_v1/` — First-Pass Hangman (Week 4) — [✅ OK]

- **Entry point:** `src/main.rs`
- **Lines of code:** 92 (incl. comments) + 65 (unit tests)
- **Dependencies:** `rand = "0.9"`
- **Concepts demonstrated:** `struct`, `impl`, associated functions, methods, `Vec<char>`, `IndexedRandom::choose`
- **Unit tests:** 8 tests covering state transitions, guess processing, display masking
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
- **Lines of code:** ~70 (incl. comments) + 50 (unit tests + helper)
- **Dependencies:** `rand = "0.9"`
- **Concepts demonstrated:** `io::stdin`, `match` on `Result` and `Ordering`, `loop`/`break`, variable shadowing, `.trim().parse()`
- **Unit tests:** 7 tests covering guess evaluation, boundary cases, input parsing
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

---

## Demo Output

### Hangman (Refined) — Sample Session

```text
$ cd scripts/hangman_refined && cargo run
Welcome to Hangman!
Word: ______
Enter your guess:
e
Word: e____e
Enter your guess:
n
Word: en____e
...
Congratulations! You guessed the word.
```

### Guessing Game — Sample Session

```text
$ cd scripts/guessing_game && cargo run
=== Guessing Game ===
I'm thinking of a number between 1 and 100.

Please input your guess:
50
You guessed: 50
Too big!

Please input your guess:
25
You guessed: 25
You win! 🎉 It took you 2 attempts.
```

### Test Suite — All Projects

```text
$ cd scripts/hangman_refined && cargo test
running 9 tests
test tests::new_game_starts_in_playing_state ... ok
test tests::correct_guess_does_not_reduce_attempts ... ok
test tests::incorrect_guess_reduces_attempts ... ok
test tests::duplicate_guess_does_not_reduce_attempts ... ok
test tests::guessing_all_letters_wins ... ok
test tests::running_out_of_attempts_loses ... ok
test tests::saturating_sub_prevents_underflow ... ok
test tests::display_word_masks_unguessed_letters ... ok
test tests::display_word_reveals_guessed_letters ... ok
test result: ok. 9 passed; 0 failed; 0 ignored

$ cd scripts/hangman_v1 && cargo test
running 8 tests
test tests::new_game_starts_in_playing_state ... ok
test tests::correct_guess_does_not_reduce_attempts ... ok
test tests::incorrect_guess_reduces_attempts ... ok
test tests::duplicate_guess_does_not_reduce_attempts ... ok
test tests::guessing_all_letters_wins ... ok
test tests::running_out_of_attempts_loses ... ok
test tests::display_word_masks_unguessed_letters ... ok
test tests::display_word_reveals_guessed_letters ... ok
test result: ok. 8 passed; 0 failed; 0 ignored

$ cd scripts/guessing_game && cargo test
running 7 tests
test tests::guess_too_small ... ok
test tests::guess_too_big ... ok
test tests::guess_correct ... ok
test tests::boundary_guess_one ... ok
test tests::boundary_guess_hundred ... ok
test tests::parse_valid_input ... ok
test tests::parse_invalid_input ... ok
test result: ok. 7 passed; 0 failed; 0 ignored
```

---

## Troubleshooting

### Common Issues

| Problem | Cause | Solution |
|---|---|---|
| `rustc: command not found` | Rust not installed | Install via `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` (Unix) or download from [rustup.rs](https://rustup.rs/) (Windows) |
| `cargo build` fails with network error | Cannot download `rand` crate | Check internet connection; run `cargo update` to refresh registry |
| `error[E0658]: use of unstable feature` | Rust version too old | Run `rustup update stable` to get latest toolchain |
| `Blocking waiting for file lock` | Another cargo process running | Wait for it to finish, or delete `~/.cargo/.package-cache` |
| Tests fail with `random` assertion | Non-deterministic test? | All tests in this portfolio are deterministic (use `game_with_word()` helpers). File an issue if reproducible. |
| `warning: unused function` | `evaluate_guess()` in guessing_game | This function is used only in tests via `#[cfg(test)]`; the warning is suppressed in test builds |

### Build From Scratch

```bash
# Full clean rebuild of all projects
cd scripts
cargo clean
cargo build --workspace
cargo test --workspace
# Expected: 24 tests passing (8 + 9 + 7), 0 failures
```
