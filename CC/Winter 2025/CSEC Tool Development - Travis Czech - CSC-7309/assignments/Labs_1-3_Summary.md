# Labs 1–3 Summary

**Course:** CSEC Tool Development (CSC-7309) | **Term:** Winter 2025 | **Instructor:** Travis Czech

---

## Overview

Three hands-on labs were assigned during Weeks 1–5, with all three reopened for catch-up during the Week 6 midterm review session. Each lab reinforced the core Rust concepts introduced in the corresponding lecture.

---

## Lab 1 — Environment & Variables (Weeks 1–2)

### Objectives

- Install Rust toolchain (Rustup, Cargo, rustc)
- Create a Cargo project and verify with `cargo run`
- Experiment with variables, mutability, and primitive types

### Key Exercises

```rust
// Immutability demonstration
let x = 5;
// x = 6;           // ← COMPILE ERROR: cannot assign to immutable variable

let mut y = 10;
y += 5;             // OK — y is mutable
println!("y = {}", y);  // → y = 15

// Type annotations
let age: u8 = 25;
let pi: f64 = 3.14159;
let initial: char = 'R';
let active: bool = true;
```

### Verification

- `rustc --version` → confirmed toolchain installation
- `cargo new lab1 && cd lab1 && cargo run` → "Hello, world!" output
- Experimented with type inference vs. explicit annotations

---

## Lab 2 — Ownership & Borrowing (Week 3)

### Objectives

- Demonstrate the three ownership rules in code
- Use references (`&T`) and mutable references (`&mut T`)
- Understand move semantics vs. `.clone()`

### Key Exercises

```rust
// Move semantics
let s1 = String::from("hello");
let s2 = s1;              // s1 is MOVED — no longer valid
// println!("{}", s1);    // ← COMPILE ERROR

// Borrowing (immutable reference)
let s3 = String::from("world");
let len = calculate_length(&s3);    // borrow, don't move
println!("'{}' is {} bytes", s3, len);

fn calculate_length(s: &String) -> usize {
    s.len()
}

// Mutable reference
let mut s4 = String::from("hello");
append_world(&mut s4);
println!("{}", s4);  // → "hello, world"

fn append_world(s: &mut String) {
    s.push_str(", world");
}
```

### Verification

- Intentionally triggered ownership errors to observe compiler messages
- Fixed errors using references, cloning, and mutable borrows
- Confirmed understanding: only one `&mut T` OR any number of `&T` at a time

---

## Lab 3 — Structs & Methods (Week 4)

### Objectives

- Define custom types with `struct`
- Implement methods via `impl` blocks
- Use `enum` for mutually exclusive states

### Key Exercises

```rust
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Self {
        Rectangle { width, height }
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn is_square(&self) -> bool {
        (self.width - self.height).abs() < f64::EPSILON
    }
}

let rect = Rectangle::new(10.0, 5.0);
println!("Area: {}", rect.area());        // → 50.0
println!("Square? {}", rect.is_square()); // → false
```

### Verification

- Created and tested struct with multiple methods
- Used associated function (`::new`) as constructor pattern
- Connected patterns to the Hangman game implementation in the same week

---

## Relationship to Portfolio Artifacts

```mermaid
graph TD
    L1[Lab 1<br/>Environment & Variables] --> L2[Lab 2<br/>Ownership & Borrowing]
    L2 --> L3[Lab 3<br/>Structs & Methods]
    L3 --> HM[Hangman Game<br/>Week 4 Midterm Project]
    L2 --> KL[Keylogger Study<br/>Week 3]

    style HM fill:#2b6cb0,color:#fff
    style KL fill:#c53030,color:#fff
```

Each lab builds directly on the previous one. Lab 3's struct patterns are immediately applied in the Week 4 Hangman project, and Lab 2's ownership concepts appear in the Week 3 keylogger study.

## Attribution

Lab design and exercises © Travis Czech / Cambrian College (CSC-7309, Winter 2025). Student summaries by Ross Moravec.
