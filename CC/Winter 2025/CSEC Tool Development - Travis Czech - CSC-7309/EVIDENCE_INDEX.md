# Evidence Index — CSC-7309 Tool Development

This index maps portfolio artifacts (code, summaries, transcripts, assignments) to course weeks and learning outcomes. Use it as a cross-reference when reviewing the repository.

> [!TIP]
> **For hiring managers:** Start with the [Midterm Project Summary](MIDTERM_PROJECT_SUMMARY.md) for the most comprehensive single artifact, then browse the code in [scripts/](scripts/).

---

## Student-Authored Code

| Artifact | Path | Week | Concepts |
|---|---|---|---|
| Hangman v1 — source | [scripts/hangman_v1/src/main.rs](scripts/hangman_v1/src/main.rs) | 4 | struct, impl, Vec, rand |
| Hangman v1 — manifest | [scripts/hangman_v1/Cargo.toml](scripts/hangman_v1/Cargo.toml) | 4 | Cargo metadata |
| Hangman refined — source (+ 9 unit tests) | [scripts/hangman_refined/src/main.rs](scripts/hangman_refined/src/main.rs) | 4 | enum, HashSet, match, saturating_sub, #[cfg(test)] |
| Hangman refined — manifest | [scripts/hangman_refined/Cargo.toml](scripts/hangman_refined/Cargo.toml) | 4 | Cargo metadata |
| Guessing Game — source | [scripts/guessing_game/src/main.rs](scripts/guessing_game/src/main.rs) | 5 | stdin, Result, Ordering, loop, parse |
| Guessing Game — manifest | [scripts/guessing_game/Cargo.toml](scripts/guessing_game/Cargo.toml) | 5 | Cargo metadata |
| Scripts inventory | [SCRIPTS_README.md](SCRIPTS_README.md) | All | — |

## Synthesized Documentation

| Document | Path | Covers |
|---|---|---|
| Course README | [README.md](README.md) | Course snapshot, schedule, navigation |
| Weekly summary | [WEEKS_1-6_RUST_FUNDAMENTALS_SUMMARY.md](WEEKS_1-6_RUST_FUNDAMENTALS_SUMMARY.md) | Weeks 1–6 concepts, commands, code, Mermaid diagrams |
| Midterm project | [MIDTERM_PROJECT_SUMMARY.md](MIDTERM_PROJECT_SUMMARY.md) | Hangman project writeup with architecture diagrams |
| Tool-dev reflection | [FINAL_PROJECT_TOOL_DEVELOPMENT.md](FINAL_PROJECT_TOOL_DEVELOPMENT.md) | Phase 1 methodology reflection + skills visualization |
| Keylogger study | [KEYLOGGER_STUDY_WEEK3.md](KEYLOGGER_STUDY_WEEK3.md) | Week 3 in-class security tool analysis |

## Assignment Writeups

| Document | Path | Week | Topic |
|---|---|---|---|
| Bug Hunt | [assignments/Assignment01_BugHunt.md](assignments/Assignment01_BugHunt.md) | 5 | Compiler-guided debugging methodology |
| Guessing Game | [assignments/Assignment02_GuessingGame.md](assignments/Assignment02_GuessingGame.md) | 5 | Rust Book Ch. 2 tutorial implementation |
| Labs 1–3 | [assignments/Labs_1-3_Summary.md](assignments/Labs_1-3_Summary.md) | 1–4 | Environment, ownership, structs |

## Visualizations (Mermaid Diagrams)

| Diagram | Location | Type |
|---|---|---|
| Course progression flowchart | Root [README.md](../../README.md) | `graph LR` |
| Skills mindmap | Root [README.md](../../README.md) | `mindmap` |
| Ownership & borrowing model | [WEEKS_1-6 Summary](WEEKS_1-6_RUST_FUNDAMENTALS_SUMMARY.md#ownership--borrowing-visual-model) | `graph TD` |
| Hangman program flow | [MIDTERM_PROJECT_SUMMARY.md](MIDTERM_PROJECT_SUMMARY.md#program-flow) | `graph TD` |
| Hangman state machine | [MIDTERM_PROJECT_SUMMARY.md](MIDTERM_PROJECT_SUMMARY.md#state-machine) | `stateDiagram-v2` |
| v1 → Refined comparison | [MIDTERM_PROJECT_SUMMARY.md](MIDTERM_PROJECT_SUMMARY.md#v1--refined-changes-improvements) | `graph LR` |
| Skills proficiency levels | [FINAL_PROJECT](FINAL_PROJECT_TOOL_DEVELOPMENT.md#skills-inventory--validated-to-week-6) | `graph LR` |
| Keylogger architecture | [KEYLOGGER_STUDY_WEEK3.md](KEYLOGGER_STUDY_WEEK3.md#architecture-overview) | `graph TD` |
| Keylogger course context | [KEYLOGGER_STUDY_WEEK3.md](KEYLOGGER_STUDY_WEEK3.md#relationship-to-other-course-content) | `graph LR` |
| Bug Hunt debug flow | [Assignment01_BugHunt.md](assignments/Assignment01_BugHunt.md#debugging-process-applied) | `graph TD` |
| Lab dependency chain | [Labs_1-3_Summary.md](assignments/Labs_1-3_Summary.md#relationship-to-portfolio-artifacts) | `graph TD` |

## Original Course Artifacts (at D:\CC, not in this repo)

| Week | Artifact | Size | Extracted? |
|---|---|---|---|
| 1 | `Transcript - CSEC Tool Development - Travis Czech.txt` | 21 KB | ✅ → WEEKS summary |
| 1 | `Links in the chat - CSEC Tool Development - Travis Czech.docx` | 15 KB | ✅ → scripts-extra/README.md |
| 2 | `TranscripCSEC Tool Development - Travis Czech - CSC-7309 - 11821 - 2025-01-15.txt` | 68 KB | ✅ → WEEKS summary |
| 3 | `CSEC Tool Development - Travis Czech - CSC-7309 - 11821 - 2025-01-22 - part 1.txt` | 39 KB | ✅ → WEEKS summary |
| 3 | `In class Simple Keylogger Win, Linux, MacOS/Claude's simple linux.docx` | 20 KB | ✅ → KEYLOGGER_STUDY_WEEK3.md |
| 3 | Video parts 2 & 3 (no transcript available) | ~245 MB | ⏳ Pending transcription |
| 4 | `Transcript - CSEC Tool Development - Travis Czech.txt` | 38 KB | ✅ → WEEKS summary |
| 4 | `Hangman_v1.txt` | 2.8 KB | ✅ → scripts/hangman_v1/ |
| 4 | `Refined Hangman with comments.txt` | 7.5 KB | ✅ → scripts/hangman_refined/ |
| 5 | `TranscripCSEC Tool Development - Travis Czech - CSC-7309 - 11821 - 2025-02-05.txt` | 1.5 KB | ✅ → WEEKS summary |
| 6 | `Transcript - CSEC Tool Development - Travis Czech - 2025-02-12.txt` | 3.9 KB | ✅ → WEEKS summary |

> [!NOTE]
> Video lectures (~1.4 GB total) and .docx binaries are intentionally **not** published to this public repository. They are retained in the private archive for reference. Published artifacts are the sanitized code extractions and student-authored synthesis documents.

## Screenshots

> [!NOTE]
> Terminal screenshots will be captured when the Rust toolchain is available in the portfolio build environment. The Mermaid diagrams above serve as the primary visual evidence for this portfolio. See the `rust-check.yml` CI workflow for automated compilation verification.

| Planned Screenshot | Description | Alternative Evidence |
|---|---|---|
| Hangman terminal session | Game running in terminal | Mermaid state machine + source code |
| `rustc --version` output | Toolchain verification | CI workflow `rust-check.yml` validates compilation |
| Compiler ownership error | Borrow checker in action | Ownership diagram + Bug Hunt writeup |
| `cargo build` output | Successful compilation | CI badge in root README |
