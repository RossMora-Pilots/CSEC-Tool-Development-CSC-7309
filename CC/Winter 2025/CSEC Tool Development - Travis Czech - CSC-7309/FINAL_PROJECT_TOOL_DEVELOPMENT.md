# Final Project — Tool Development Methodology & Phase 1 Portfolio

**Course:** CSEC Tool Development (CSC-7309) | **Term:** Winter 2025 | **Instructor:** Travis Czech

> [!NOTE]
> **Portfolio Scope:** This document covers **Phase 1** (Weeks 1–6, pre-midterm) of the CSEC Tool Development course. It presents the foundational Rust skills, security-tool methodology, and applied projects completed during the first half of the course. Phase 2 content (Weeks 7–12: network scanners, cryptography, advanced tool development) will extend this portfolio as materials become available.

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

| Week | Exercise | Concepts Reinforced | Outcome | Evidence |
|---|---|---|---|---|
| 1 | Hello, World (cargo new) | Toolchain end-to-end | ✅ Compiled & ran | [Lab 1](assignments/Labs_1-3_Summary.md) |
| 2 | Variables & types walkthrough | Mutability, type inference | ✅ Demonstrated | [Lab 1](assignments/Labs_1-3_Summary.md) |
| 3 | In-class simple keylogger study | Ownership in practical code | ✅ Reviewed | [Keylogger Study](KEYLOGGER_STUDY_WEEK3.md) |
| 3 | Ownership & borrowing exercises | Move, borrow, mutable borrow | ✅ Completed | [Lab 2](assignments/Labs_1-3_Summary.md) |
| 4 | Hangman v1 (live-coded) | Structs, methods, Vec | ✅ Working | [Source](scripts/hangman_v1/src/main.rs) |
| 4 | Hangman refined (refactor) | Enums, HashSet, idiomatic Rust | ✅ Working + tested | [Source](scripts/hangman_refined/src/main.rs) |
| 5 | Bug Hunt (Assignment 1) | Compiler diagnostics, debugging | ✅ Completed | [Writeup](assignments/Assignment01_BugHunt.md) |
| 5 | Guessing Game (Rust Book ch. 2) | stdin, match, loop, parse | ✅ Completed | [Source](scripts/guessing_game/src/main.rs) |
| 6 | Practice midterm | Sections 1–5 synthesis | ✅ Completed | [Labs 1–3](assignments/Labs_1-3_Summary.md) |

## Skills Inventory — Validated to Week 6

```mermaid
%%{init: {'theme': 'base', 'themeVariables': {'primaryColor': '#2b6cb0'}}}%%
graph LR
    subgraph Proficient
        A[Cargo Workflow]
        B[Rust Syntax]
        C[Documentation]
    end
    subgraph Intermediate
        D[Memory Safety<br/>Reasoning]
        E[Structs & Enums]
        F[Pattern Matching]
    end
    subgraph Beginner+
        G[stdlib Collections]
        H[I/O & Parsing]
        I[External Crates]
    end
    subgraph Conceptual
        J[Security-Tool<br/>Architecture]
        K[Concurrency]
        L[Networking]
    end

    style Proficient fill:#38a169,color:#fff,stroke:#2f855a
    style Intermediate fill:#2b6cb0,color:#fff,stroke:#2c5282
    style Beginner+ fill:#d69e2e,color:#fff,stroke:#b7791f
    style Conceptual fill:#718096,color:#fff,stroke:#4a5568
```

| Skill Area | Level | Evidence |
|---|---|---|
| **Rust syntax** | ██████████ Proficient | Hangman refined + Guessing Game + 9 unit tests |
| **Memory safety reasoning** | ████████░░ Intermediate | Ownership/borrowing labs + keylogger study |
| **Cargo workflow** | ██████████ Proficient | 3 multi-crate projects with build automation |
| **Pattern matching** | ████████░░ Intermediate | `match` on enum, Result, Ordering |
| **stdlib collections** | ██████░░░░ Beginner+ | `Vec`, `HashSet` used correctly |
| **External crates** | ██████░░░░ Beginner+ | `rand`, `chrono`, `evdev` studied |
| **I/O & stdin parsing** | ██████░░░░ Beginner+ | Hangman + Guessing Game input loops |
| **Error handling** | ██████░░░░ Beginner+ | `.expect()`, `match Result`, `?` operator |
| **Security-tool architecture** | ████░░░░░░ Conceptual | Keylogger study + scanner previews |

## Reflection

The first half of CSEC Tool Development succeeds because it treats Rust not as a language to memorize, but as a **design philosophy**. Ownership is the lens; every subsequent concept (borrowing, lifetimes, traits, concurrency) sits on that foundation.

Whether the course continued into network scanners, crypto tools, or malware analysis, the Week 1–6 foundation was designed to make those topics *tractable* rather than magical.

**Personal takeaway:** I now read Rust compiler errors as helpful feedback rather than gatekeeping. The v1 → refined Hangman exercise was the moment that clicked. Given a real security-tool problem today, I would reach for Rust without hesitation for anything that touches memory, threads, or a network socket.

## Phase 2 Roadmap

This portfolio is actively maintained. The following items represent planned extensions as additional course content becomes available:

| Phase | Content | Status |
|---|---|---|
| **Phase 1** (this portfolio) | Weeks 1–6: Rust fundamentals, ownership, structs, Hangman, keylogger study | ✅ Complete |
| **Phase 2a** | Weeks 7–9: Port scanner implementation, networking primitives | 🔜 Planned |
| **Phase 2b** | Weeks 10–11: Vulnerability scanner, cryptography modules | 🔜 Planned |
| **Phase 2c** | Week 12: Final project — integrated security tool | 🔜 Planned |

> [!TIP]
> **For hiring managers:** Phase 1 demonstrates the complete Rust foundation needed for security tool development. The skills validated here (ownership model, struct-based architecture, compiler-guided development, safe concurrency patterns) directly transfer to the advanced tools in Phase 2.

## Attribution

Course design, methodology, and all lecture content © Travis Czech / Cambrian College (CSC-7309, Winter 2025). This reflection is student-authored synthesis by Ross Moravec for portfolio purposes.
