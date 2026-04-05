# CSEC Tool Development — Travis Czech — CSC-7309

![Portfolio CI](https://github.com/RossMora/407-tool-development/actions/workflows/portfolio-ci.yml/badge.svg)

> **Start Here:** course overview, learning outcomes, and navigation for the CSEC Tool Development (CSC-7309) public portfolio.

## Course Snapshot

| Attribute | Value |
|---|---|
| **Course Code** | CSC-7309 (Section 11821-002) |
| **Course Title** | CSEC Tool Development |
| **Program** | Postgraduate Cybersecurity Certificate |
| **Institution** | Cambrian College, Sudbury, Ontario |
| **Term** | Winter 2025 (January – February 2025) |
| **Instructor** | Travis Czech |
| **Language** | Rust (Rustup + Cargo) |
| **Modality** | Live lectures with live-coding demonstrations |
| **Content Scope (this portfolio)** | Weeks 1 – 6 (pre-midterm) |

## Quick Links

| Document | Description |
|---|---|
| [WEEKS_1-6_RUST_FUNDAMENTALS_SUMMARY.md](WEEKS_1-6_RUST_FUNDAMENTALS_SUMMARY.md) | Synthesized weekly concepts with Mermaid diagrams |
| [MIDTERM_PROJECT_SUMMARY.md](MIDTERM_PROJECT_SUMMARY.md) | Hangman game writeup with architecture diagrams + metrics |
| [KEYLOGGER_STUDY_WEEK3.md](KEYLOGGER_STUDY_WEEK3.md) | 🔒 In-class keylogger security analysis (Week 3) |
| [FINAL_PROJECT_TOOL_DEVELOPMENT.md](FINAL_PROJECT_TOOL_DEVELOPMENT.md) | Phase 1 methodology reflection + skills visualization |
| [EVIDENCE_INDEX.md](EVIDENCE_INDEX.md) | Complete artifact index with 11 Mermaid diagrams |
| [SCRIPTS_README.md](SCRIPTS_README.md) | Rust source code inventory (3 Cargo projects) |
| [assignments/](assignments/) | Assignment writeups (Bug Hunt, Guessing Game, Labs 1–3) |
| [scripts/](scripts/) | Student-authored Rust source (Hangman v1 + refined + Guessing Game) |
| [scripts-extra/](scripts-extra/) | External/reference materials + instructor-shared URLs |

## Weekly Schedule (12-Week Course, Content Available Weeks 1–6)

| Week | Date | Topic | Lab / Deliverable |
|---|---|---|---|
| **Week 1** | 2025-01-08 | Development Environment Setup (Rustup, Cargo, VS Code) | Hello World via `cargo new` |
| **Week 2** | 2025-01-15 | Variables, Mutability, Data Types | Type-exploration code walkthrough |
| **Week 3** | 2025-01-22 | Ownership, Borrowing, References (3-part lecture) | In-class Simple Keylogger study |
| **Week 4** | 2025-01-29 | Structs, Methods, Enums, Cryptography intro | Hangman game implementation |
| **Week 5** | 2025-02-05 | Assignment lab — "Bug Hunt" + Rust book guessing-game tutorial | Self-paced debugging |
| **Week 6** | 2025-02-12 | Midterm preparation & review (sections 1–5) | Practice midterm + labs 1–3 reopened |
| _Weeks 7–12_ | _(not in this portfolio)_ | Continuation topics: advanced tool dev, crypto, networking | — |

## Learning Outcomes Demonstrated

By the end of Week 6, the following outcomes are documented in this portfolio:

1. **Environment Competency** — Installing and verifying a Rust toolchain cross-platform
2. **Language Fundamentals** — Using Rust's strong type system, mutability rules, and primitives
3. **Ownership Model Fluency** — Explaining and applying the three ownership rules and borrowing
4. **Data Modeling** — Composing custom types with `struct` and `enum`; implementing methods via `impl`
5. **Program Construction** — Writing a complete, interactive terminal application (Hangman) from scratch
6. **Security Posture** — Developing in isolated VMs; distinguishing defensive vs. offensive use cases

## Naming Conventions

- **Folder:** `CC/<Term>/<Course Name - Instructor - Code>`
- **Screenshots:** `wkNN_<topic>_<index>.png` (e.g., `wk04_hangman_1.png`) or `ScreenshotN_ShortDesc.png`
- **Scripts:** student-authored Rust in `scripts/`; reference material in `scripts-extra/`
- **Transcripts:** sanitized excerpts referenced in `EVIDENCE_INDEX.md`

## Tools & Technologies Covered

| Category | Tools |
|---|---|
| **Language** | Rust (2024 edition), Python (prerequisite reference) |
| **Toolchain** | Rustup, Cargo, `rustc`, `rust-analyzer` |
| **Editor** | Visual Studio Code + Rust extension |
| **Version Control** | Git + GitHub |
| **Runtime Crates** | `rand` (randomness), `std::io`, `std::collections::HashSet` |
| **Security Tooling Concepts** | Keyloggers (study), port/vulnerability scanners (preview) |

## Attribution & Responsible Use

- **Original course content, lectures, and exercises:** © Travis Czech / Cambrian College, 2025
- **Student-authored summaries, annotations, and adaptations:** Ross Moravec, 2025

> [!CAUTION]
> Security-tool concepts (e.g., keylogger study, scanner previews) are academic and intended strictly for **defensive learning**. Do not use against systems you do not own or lack explicit authorization to test. See [KEYLOGGER_STUDY_WEEK3.md](KEYLOGGER_STUDY_WEEK3.md) for the full responsible-use framework.
