# Assignment 1 — Bug Hunt (Week 5)

**Course:** CSEC Tool Development (CSC-7309) | **Week:** 5 | **Date:** 2025-02-05 | **Instructor:** Travis Czech

---

## Assignment Description

Students received pre-written Rust code containing **deliberate bugs** and were tasked with:

1. Compiling the code and reading the compiler diagnostics
2. Identifying and fixing each issue methodically
3. Running the corrected program and verifying output

## Methodology

> [!TIP]
> The Bug Hunt exercise reinforces the **"read the compiler error → understand → fix"** methodology that is central to Rust development. Unlike C/C++ where compiler errors can be cryptic, Rust's compiler (`rustc`) provides specific, actionable error messages.

### Common Bug Categories Encountered

| Bug Type | Rust Compiler Error | Fix |
|---|---|---|
| **Immutable variable reassignment** | `cannot assign twice to immutable variable` | Add `mut` keyword: `let mut x = 5;` |
| **Ownership move after use** | `value used here after move` | Use `.clone()` or pass by reference (`&`) |
| **Type mismatch** | `expected i32, found &str` | Parse string to integer: `.parse::<i32>()` |
| **Missing `mut` on reference** | `cannot borrow as mutable` | Change `&self` to `&mut self` |
| **Unused variable** | `unused variable: x` (warning) | Prefix with `_` or remove |
| **Missing match arms** | `non-exhaustive patterns` | Add all enum variants to `match` |

### Debugging Process (Applied)

```mermaid
graph TD
    A[Write / Receive Code] --> B[cargo build]
    B --> C{Compiles?}
    C -->|No| D[Read Error Message]
    D --> E[Identify Bug Category]
    E --> F[Apply Fix]
    F --> B
    C -->|Yes| G[cargo run]
    G --> H{Correct Output?}
    H -->|No| I[Add println! Debug]
    I --> A
    H -->|Yes| J[✅ Done]

    style D fill:#c53030,color:#fff
    style J fill:#38a169,color:#fff
```

## Example: Fixing an Ownership Bug

**Before (buggy):**

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;           // s1 moved to s2
    println!("{}", s1);    // ERROR: value used after move
}
```

**Compiler output:**

```text
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:4:20
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`
3 |     let s2 = s1;
  |              -- value moved here
4 |     println!("{}", s1);
  |                    ^^ value borrowed here after move
```

**After (fixed):**

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();       // deep copy — s1 retained
    println!("{}", s1);        // OK
    println!("{}", s2);        // OK
}
```

## Concepts Reinforced

- **Compiler-guided development** — Rust's error messages are detailed enough to serve as a learning tool
- **Ownership rules** — Every move, borrow, and lifetime error traces back to the three ownership rules
- **Iterative fix-compile-test loop** — The core development workflow for systems programming
- **Pattern matching completeness** — The compiler enforces exhaustive `match` coverage

## Learning Outcome

The Bug Hunt exercise demonstrated that Rust's compiler is not an obstacle — it is a **teaching tool and safety net**. In a security context, the same compiler checks that catch these educational bugs also prevent the memory-safety vulnerabilities (buffer overflows, use-after-free, data races) that plague C/C++ security tools.

---

## My Implementation & Challenges

### Debugging Story: The Borrow Checker Surprise

The most instructive bug I encountered was not from the provided code, but from my own attempt to "fix" one of the exercises. I initially tried to fix a move-after-use bug by storing a reference instead of cloning:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = &s1;        // I thought borrowing would fix it
    let s3 = s1;          // But this MOVES s1...
    println!("{}", s2);   // ERROR: s1 was moved while s2 still borrows it
}
```

**Compiler output I got:**

```text
error[E0505]: cannot move out of `s1` because it is borrowed
 --> src/main.rs:4:14
  |
3 |     let s2 = &s1;
  |              --- borrow of `s1` occurs here
4 |     let s3 = s1;
  |              ^^ move out of `s1` occurs here
5 |     println!("{}", s2);
  |                    -- borrow later used here
```

**What I learned:** You cannot move a value while there are outstanding borrows. The fix was either to `.clone()` before the borrow, or to restructure the code so the borrow ends before the move. This was the moment the borrowing rules went from abstract theory to concrete understanding.

### Time Spent

Approximately 1.5 hours — most of that time was spent intentionally triggering additional compiler errors beyond the assigned bugs, to deepen my understanding of the error messages.

## Attribution

Assignment design © Travis Czech / Cambrian College (CSC-7309, Week 5, 2025-02-05). Student writeup by Ross Moravec.

## Competencies Achieved

- [x] Read and interpret Rust compiler error messages (`E0382`, `E0505`, `E0308`)
- [x] Identify ownership violations (move after use, borrow while moved)
- [x] Apply fixes: `.clone()`, pass by reference (`&`), add `mut`
- [x] Distinguish between immutable and mutable borrows
- [x] Use the iterative `compile → read error → fix → compile` debugging workflow
- [x] Understand how compiler-guided debugging prevents security vulnerabilities
