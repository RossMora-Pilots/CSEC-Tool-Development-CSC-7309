# Evidence Index — CSC-7309 Tool Development

This index maps portfolio artifacts (code, summaries, transcripts, assignments) to course weeks and learning outcomes. Use it as a cross-reference when reviewing the repository.

> [!TIP]
> **For hiring managers:** Start with the [Midterm Project Summary](MIDTERM_PROJECT_SUMMARY.md) for the most comprehensive single artifact, then browse the code in [scripts/](scripts/).

---

## Student-Authored Code

| Artifact | Path | Week | Concepts |
|---|---|---|---|
| Hangman v1 — source (+ 8 unit tests) | [scripts/hangman_v1/src/main.rs](scripts/hangman_v1/src/main.rs) | 4 | struct, impl, Vec, rand |
| Hangman v1 — manifest | [scripts/hangman_v1/Cargo.toml](scripts/hangman_v1/Cargo.toml) | 4 | Cargo metadata |
| Hangman refined — source (+ 9 unit tests) | [scripts/hangman_refined/src/main.rs](scripts/hangman_refined/src/main.rs) | 4 | enum, HashSet, match, saturating_sub, #[cfg(test)] |
| Hangman refined — manifest | [scripts/hangman_refined/Cargo.toml](scripts/hangman_refined/Cargo.toml) | 4 | Cargo metadata |
| Guessing Game — source (+ 7 unit tests) | [scripts/guessing_game/src/main.rs](scripts/guessing_game/src/main.rs) | 5 | stdin, Result, Ordering, loop, parse |
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
| Course progression flowchart | Root [README.md](../../../README.md) | `graph LR` |
| Skills mindmap | Root [README.md](../../../README.md) | `mindmap` |
| Repository navigation | Root [README.md](../../../README.md) | `graph TD` |
| Ownership & borrowing model | [WEEKS_1-6 Summary](WEEKS_1-6_RUST_FUNDAMENTALS_SUMMARY.md#ownership--borrowing-visual-model) | `graph TD` |
| Hangman program flow | [MIDTERM_PROJECT_SUMMARY.md](MIDTERM_PROJECT_SUMMARY.md#program-flow) | `graph TD` |
| Hangman state machine | [MIDTERM_PROJECT_SUMMARY.md](MIDTERM_PROJECT_SUMMARY.md#state-machine) | `stateDiagram-v2` |
| v1 → Refined comparison | [MIDTERM_PROJECT_SUMMARY.md](MIDTERM_PROJECT_SUMMARY.md#v1--refined-changes-improvements) | `graph LR` |
| Hangman data flow | [MIDTERM_PROJECT_SUMMARY.md](MIDTERM_PROJECT_SUMMARY.md#data-flow) | `graph TD` |
| Learning timeline | [FINAL_PROJECT](FINAL_PROJECT_TOOL_DEVELOPMENT.md#learning-timeline) | `gantt` |
| Methodology flow | [FINAL_PROJECT](FINAL_PROJECT_TOOL_DEVELOPMENT.md#instructor-methodology-flow) | `graph LR` |
| Skills proficiency levels | [FINAL_PROJECT](FINAL_PROJECT_TOOL_DEVELOPMENT.md#skills-inventory--validated-to-week-6) | `graph LR` |
| Skill distribution | [FINAL_PROJECT](FINAL_PROJECT_TOOL_DEVELOPMENT.md#skill-acquisition-distribution) | `pie` |
| Keylogger architecture | [KEYLOGGER_STUDY_WEEK3.md](KEYLOGGER_STUDY_WEEK3.md#architecture-overview) | `graph TD` |
| Keylogger event loop | [KEYLOGGER_STUDY_WEEK3.md](KEYLOGGER_STUDY_WEEK3.md#event-loop-sequence) | `sequenceDiagram` |
| Keylogger course context | [KEYLOGGER_STUDY_WEEK3.md](KEYLOGGER_STUDY_WEEK3.md#curriculum-context) | `graph LR` |
| Bug Hunt debug flow | [Assignment01_BugHunt.md](assignments/Assignment01_BugHunt.md#debugging-process-applied) | `graph TD` |
| Lab dependency chain | [Labs_1-3_Summary.md](assignments/Labs_1-3_Summary.md#relationship-to-portfolio-artifacts) | `graph TD` |
| Toolchain setup flow | [WEEKS_1-6 Summary](WEEKS_1-6_RUST_FUNDAMENTALS_SUMMARY.md#week-1) | `graph LR` |
| Type system hierarchy | [WEEKS_1-6 Summary](WEEKS_1-6_RUST_FUNDAMENTALS_SUMMARY.md#week-2) | `graph TD` |
| Struct → impl → enum | [WEEKS_1-6 Summary](WEEKS_1-6_RUST_FUNDAMENTALS_SUMMARY.md#week-4) | `graph LR` |
| Guessing game state machine | [WEEKS_1-6 Summary](WEEKS_1-6_RUST_FUNDAMENTALS_SUMMARY.md#week-5) | `stateDiagram-v2` |

## Original Course Artifacts (at D:\CC, not in this repo)

| Week | Artifact | Size | Extracted? |
|---|---|---|---|
| 1 | `Transcript - CSEC Tool Development - Travis Czech.txt` | 21 KB | ✅ → WEEKS summary |
| 1 | `Links in the chat - CSEC Tool Development - Travis Czech.docx` | 15 KB | ✅ → scripts-extra/README.md |
| 2 | `TranscripCSEC Tool Development - Travis Czech - CSC-7309 - 11821 - 2025-01-15.txt` | 68 KB | ✅ → WEEKS summary |
| 3 | `CSEC Tool Development - Travis Czech - CSC-7309 - 11821 - 2025-01-22 - part 1.txt` | 39 KB | ✅ → WEEKS summary |
| 3 | `In class Simple Keylogger Win, Linux, MacOS/Claude's simple linux.docx` | 20 KB | ✅ → KEYLOGGER_STUDY_WEEK3.md |
| 3 | Video part 2 (`.mp4`, ~11 MB) | 11 MB | ❌ Corrupt — moov atom missing (truncated recording) |
| 3 | Video part 3 (`.mp4`, ~234 MB) | 234 MB | ❌ No speech — 40 min silent screen-share (confirmed via Whisper + volume analysis) |
| 4 | `Transcript - CSEC Tool Development - Travis Czech.txt` | 38 KB | ✅ → WEEKS summary |
| 4 | Video part 2 (`.mp4`) | ~200 MB | ✅ → Transcribed (79 lines, Whisper base model) |
| 4 | `Hangman_v1.txt` | 2.8 KB | ✅ → scripts/hangman_v1/ |
| 4 | `Refined Hangman with comments.txt` | 7.5 KB | ✅ → scripts/hangman_refined/ |
| 5 | `TranscripCSEC Tool Development - Travis Czech - CSC-7309 - 11821 - 2025-02-05.txt` | 1.5 KB | ✅ → WEEKS summary |
| 6 | `Transcript - CSEC Tool Development - Travis Czech - 2025-02-12.txt` | 3.9 KB | ✅ → WEEKS summary |

> [!NOTE]
> Video lectures (~1.4 GB total) and .docx binaries are intentionally **not** published to this public repository. They are retained in the private archive for reference. Published artifacts are the sanitized code extractions and student-authored synthesis documents.

## Screenshots & Compilation Evidence

> [!NOTE]
> While the `screenshots/` directory does not contain image files (this portfolio uses text-based evidence), **execution evidence is embedded directly in the documents** as terminal output transcripts — a more verifiable and diff-friendly format than screenshots.

### Embedded Execution Evidence

| Evidence Type | Location | Description |
|---|---|---|
| Hangman v1 game session | [SCRIPTS_README.md](SCRIPTS_README.md#demo-output--execution-evidence) | Full terminal session: word selection → guesses → win/loss |
| Hangman refined game session | [SCRIPTS_README.md](SCRIPTS_README.md#demo-output--execution-evidence) | Interactive play with improved UI feedback |
| Guessing game session | [SCRIPTS_README.md](SCRIPTS_README.md#demo-output--execution-evidence) | Number guessing with attempt counter |
| Full test suite output | [SCRIPTS_README.md](SCRIPTS_README.md#demo-output--execution-evidence) | `cargo test` results: 24/24 tests passing |
| Assignment 2 demo | [Assignment02_GuessingGame.md](assignments/Assignment02_GuessingGame.md#demo-output) | Guessing game with attempt tracking |
| Midterm demo | [MIDTERM_PROJECT_SUMMARY.md](MIDTERM_PROJECT_SUMMARY.md#demo-session) | Hangman refined interactive play |

### Automated Verification

| Evidence | Description | Status |
|---|---|---|
| `cargo check` (3 projects) | All projects compile cleanly | ✅ Verified locally |
| `cargo test` (hangman_refined) | 9/9 unit tests pass | ✅ Verified locally |
| `cargo test` (hangman_v1) | 8/8 unit tests pass | ✅ Added in remediation cycle 2 |
| `cargo test` (guessing_game) | 7/7 unit tests pass | ✅ Added in remediation cycle 2 |
| `rust-check.yml` CI workflow | Automated compilation + test on push | ✅ Workflow configured |
| Mermaid diagrams | Visual evidence of architecture & flow | ✅ 21 diagrams in portfolio |
