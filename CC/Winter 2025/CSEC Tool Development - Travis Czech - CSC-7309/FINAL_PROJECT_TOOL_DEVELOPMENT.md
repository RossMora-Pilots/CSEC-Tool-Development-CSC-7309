# Final Project — Scoped Reflection on Tool Development

**Course:** CSEC Tool Development (CSC-7309) | **Term:** Winter 2025 | **Instructor:** Travis Czech

> **Scope Notice:** This portfolio covers **Weeks 1–6** (pre-midterm). The second half of the course, including the formal final project, falls outside the content available in this snapshot. This document instead presents a **scoped reflection** on tool development methodology as taught and practiced through Week 6, and sketches the tool-development arc the course was pointing toward.

---

## What Was Covered (Weeks 1–6)

By the midterm, the course had built the full Rust foundation needed to author custom security tools:

- Environment mastery (Rustup, Cargo, VS Code)
- Language fundamentals (variables, types, mutability)
- Memory safety model (ownership, borrowing, references)
- Data modeling (structs, impls, methods, enums)
- One complete working program (Hangman, with refactor to idiomatic Rust)

## What the Course Was Building Toward

From the Week 1 and Week 2 lectures, the instructor previewed the trajectory:

| Tool Type | Rust Concepts Needed | When in Course |
|---|---|---|
| **Hangman game** (completed) | structs, enums, Vec, HashSet, match, rand | Week 4 |
| **Port scanner** (mentioned Week 2) | `std::net::TcpStream`, threading, error handling | Post-midterm |
| **Vulnerability scanner** (preview) | HTTP clients, parsers, concurrency | Post-midterm |
| **Simple keylogger** (study ref, Week 3) | OS-specific crates, raw input handling | Post-midterm |
| **Cryptographic tools** (preview, Week 4) | byte arrays, trait objects, `crypto` crates | Post-midterm |

## Methodology Observed

Throughout the course the instructor reinforced a consistent methodology for security tool development:

### 1. Build Your Own, When You Can

> *"I personally like writing my own tools instead of using somebody else's. Makes life a little bit easier."* — Travis Czech, Week 1

Using custom tooling lets you:

- Understand exactly what the tool does (no surprises from third-party code)
- Adapt it to novel scenarios
- Learn systems concepts that make you a better defender

### 2. Use VMs for Security Work

> *"I recommend using either Linux or a virtualized version of Windows."* — Travis Czech, Week 1

Security tools can accidentally damage the host. Always develop in VMs that can be reverted from snapshot.

### 3. Live-Code, Don't Read Slides

The instructor's delivery was live-coded. This:

- Exposed real compiler errors and typical fixes
- Demonstrated iterative thinking
- Embedded the mental model of "write, compile, read the error, fix" into the students

### 4. Refactor After It Works

The v1 → refined Hangman pattern (see [MIDTERM_PROJECT_SUMMARY.md](MIDTERM_PROJECT_SUMMARY.md)) is the core methodology: get a working solution first, then refactor to idiomatic code once the problem is understood.

### 5. Respect the Compiler

> The borrow checker feels adversarial until you realize it's preventing exactly the bug classes (use-after-free, data races) that plague C and C++.

## Applied Exercises Summary

| Week | Exercise | Concepts Reinforced | Outcome |
|---|---|---|---|
| 1 | Hello, World (cargo new) | Toolchain end-to-end | ✅ Compiled & ran |
| 2 | Variables & types walkthrough | Mutability, type inference | ✅ Demonstrated |
| 3 | In-class simple keylogger study | Ownership in practical code | ✅ Reviewed |
| 4 | Hangman v1 (live-coded) | Structs, methods, Vec | ✅ Working |
| 4 | Hangman refined (refactor) | Enums, HashSet, idiomatic Rust | ✅ Working |
| 5 | Bug Hunt (Assignment 1) | Compiler diagnostics, debugging | ⏳ Completed |
| 5 | Guessing Game (Rust Book ch. 2) | stdin, match, loop, parse | ⏳ Completed |
| 6 | Practice midterm | Sections 1–5 synthesis | ⏳ Completed |

## Skills Inventory — Validated to Week 6

| Skill Area | Level Reached | Evidence |
|---|---|---|
| **Rust syntax** | Intermediate | Hangman refined implementation |
| **Memory safety reasoning** | Intermediate | Ownership/borrowing discussions |
| **Cargo workflow** | Proficient | Multi-crate project structure |
| **stdlib collections** | Beginner+ | `Vec`, `HashSet` used correctly |
| **External crates** | Beginner | `rand` crate integrated |
| **I/O & stdin parsing** | Beginner+ | Hangman input loop |
| **Error handling** | Beginner | `.expect()`, `match Result` |
| **Security-tool architecture** | Conceptual | Discussed, not yet implemented |

## Reflection

The first half of CSEC Tool Development succeeds because it treats Rust not as a language to memorize, but as a **design philosophy**. Ownership is the lens; every subsequent concept (borrowing, lifetimes, traits, concurrency) sits on that foundation.

Whether the course continued into network scanners, crypto tools, or malware analysis, the Week 1–6 foundation was designed to make those topics *tractable* rather than magical.

**Personal takeaway:** I now read Rust compiler errors as helpful feedback rather than gatekeeping. The v1 → refined Hangman exercise was the moment that clicked. Given a real security-tool problem today, I would reach for Rust without hesitation for anything that touches memory, threads, or a network socket.

## Future Work

When additional course content becomes available, this portfolio should be extended with:

- [ ] Post-midterm lecture summaries (Weeks 7–12)
- [ ] Port scanner implementation (if covered)
- [ ] Vulnerability scanner writeup (if covered)
- [ ] Cryptography module notes (if covered)
- [ ] Formal final-project writeup (replacing this scoped reflection)

## Attribution

Course design, methodology, and all lecture content © Travis Czech / Cambrian College (CSC-7309, Winter 2025). This reflection is student-authored synthesis by Ross Moravec for portfolio purposes.
