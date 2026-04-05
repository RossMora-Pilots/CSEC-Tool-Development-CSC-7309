# Evidence Index — CSC-7309 Tool Development

This index maps portfolio artifacts (code, summaries, transcripts, screenshots) to course weeks and learning outcomes. Use it as a cross-reference when reviewing the repository.

---

## Student-Authored Code

| Artifact | Path | Week | Concepts |
|---|---|---|---|
| Hangman v1 — source | [scripts/hangman_v1/src/main.rs](scripts/hangman_v1/src/main.rs) | 4 | struct, impl, Vec, rand |
| Hangman v1 — manifest | [scripts/hangman_v1/Cargo.toml](scripts/hangman_v1/Cargo.toml) | 4 | Cargo metadata |
| Hangman refined — source | [scripts/hangman_refined/src/main.rs](scripts/hangman_refined/src/main.rs) | 4 | enum, HashSet, match, saturating_sub |
| Hangman refined — manifest | [scripts/hangman_refined/Cargo.toml](scripts/hangman_refined/Cargo.toml) | 4 | Cargo metadata |
| Scripts inventory | [SCRIPTS_README.md](SCRIPTS_README.md) | All | — |

## Synthesized Documentation

| Document | Path | Covers |
|---|---|---|
| Course README | [README.md](README.md) | Course snapshot, schedule, navigation |
| Weekly summary | [WEEKS_1-6_RUST_FUNDAMENTALS_SUMMARY.md](WEEKS_1-6_RUST_FUNDAMENTALS_SUMMARY.md) | Weeks 1–6 concepts, commands, code |
| Midterm project | [MIDTERM_PROJECT_SUMMARY.md](MIDTERM_PROJECT_SUMMARY.md) | Hangman project writeup |
| Tool-dev reflection | [FINAL_PROJECT_TOOL_DEVELOPMENT.md](FINAL_PROJECT_TOOL_DEVELOPMENT.md) | Scoped final reflection |

## Original Course Artifacts (at D:\CC, not in this repo)

| Week | Artifact | Size | Notes |
|---|---|---|---|
| 1 | `Transcript - CSEC Tool Development - Travis Czech.txt` | 21 KB | Lecture transcript (2025-01-08) |
| 1 | `Links in the chat - CSEC Tool Development - Travis Czech.docx` | 15 KB | Chat-shared reference URLs |
| 2 | `TranscripCSEC Tool Development - Travis Czech - CSC-7309 - 11821 - 2025-01-15.txt` | 68 KB | Lecture transcript |
| 3 | `CSEC Tool Development - Travis Czech - CSC-7309 - 11821 - 2025-01-22 - part 1.txt` | 39 KB | Lecture transcript pt. 1 |
| 3 | `In class Simple Keylogger Win, Linux, MacOS/Claude's simple linux.docx` | 20 KB | In-class keylogger study reference |
| 4 | `Transcript - CSEC Tool Development - Travis Czech.txt` | 38 KB | Lecture transcript (2025-01-29) |
| 4 | `Hangman_v1.txt` | 2.8 KB | **Extracted → `scripts/hangman_v1/`** |
| 4 | `Refined Hangman with comments.txt` | 7.5 KB | **Extracted → `scripts/hangman_refined/`** |
| 5 | `TranscripCSEC Tool Development - Travis Czech - CSC-7309 - 11821 - 2025-02-05.txt` | 1.5 KB | Short lab-session transcript |
| 6 | `Transcript - CSEC Tool Development - Travis Czech - 2025-02-12.txt` | 3.9 KB | Review-session transcript |

Source location: `D:\CC\Winter2025\CSEC Tool Development - Travis Czech - CSC-7309 - 11821 - - 202501 - 002\`

**Note:** Video lectures (~1.4 GB total) and .docx binaries are intentionally **not** published to this public repository. They are retained in the private archive for reference. Published artifacts are the sanitized code extractions and the student-authored synthesis documents.

## Screenshots (planned)

Evidence images to capture as the portfolio matures:

| Filename (planned) | Description | Status |
|---|---|---|
| `screenshots/wk01_rustup_install.png` | Rustup + cargo version verification | Not captured |
| `screenshots/wk01_cargo_new.png` | First `cargo new` + `cargo run` output | Not captured |
| `screenshots/wk02_variables.png` | Variables, mutability, type annotations | Not captured |
| `screenshots/wk03_ownership.png` | Compiler enforcing ownership rules | Not captured |
| `screenshots/wk04_hangman_run.png` | Hangman terminal session (refined) | Not captured |
| `screenshots/wk05_bughunt.png` | Bug Hunt before/after compilation fix | Not captured |
| `screenshots/wk06_practice_midterm.png` | Completed practice midterm | Not captured |

When screenshots are added, update the `## Screenshots` section below with captions.

## Screenshots

_None yet captured. Add entries grouped by week, following the pattern:_

```
## Week NN — Topic
- [filename.png](screenshots/filename.png) — Short descriptive caption
```
