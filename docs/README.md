# Pilot 407 — Documentation Index

Internal documentation for **Pilot 407: CSEC Tool Development (CSC-7309) Portfolio**. This folder holds pilot-level operations docs. The consumer-facing portfolio content lives under [CC/Winter 2025/...](../CC/Winter%202025/CSEC%20Tool%20Development%20-%20Travis%20Czech%20-%20CSC-7309/).

## Quick Links

| Document | Purpose |
|---|---|
| [Runbook.md](Runbook.md) | Daily operations for this pilot and its public repo |
| [Checklist.md](Checklist.md) | Near-term task tracking (complements ROADMAP.md) |
| [sessions.md](sessions.md) | Auto-generated index of work sessions (SHA256-hashed) |
| [PORTFOLIO_SCRUTINY_REPORT.md](PORTFOLIO_SCRUTINY_REPORT.md) | Employer-perspective assessment (8.4/10) + 27-item remediation log |
| [WORKFLOWS.md](WORKFLOWS.md) | CI/CD pipeline architecture and workflow documentation |
| [INSTRUCTOR_ATTRIBUTION_REQUEST.md](INSTRUCTOR_ATTRIBUTION_REQUEST.md) | Draft attribution/license request for instructor |
| [../ROADMAP.md](../ROADMAP.md) | Now / Next / Later / Milestones for this pilot |
| [../AGENTS.md](../AGENTS.md) | Agent/session entry point |
| [../CONTRIBUTING.md](../CONTRIBUTING.md) | PM conventions |

## Public Repository Mapping

- **Local pilot:** `D:\pilots\407-Tool-Development`
- **Source content (private):** `D:\CC\Winter2025\CSEC Tool Development - Travis Czech - CSC-7309 - 11821 - - 202501 - 002\`
- **Target public repo:** `RossMora/407-tool-development` (GitHub, Public, main branch, Actions enabled, Issues enabled, Discussions disabled)

## Portfolio Health Summary

| Metric | Value |
|---|---|
| Overall score | 9.1/10 (post-remediation) |
| Mermaid diagrams | 21 (7 types: graph, stateDiagram, mindmap, gantt, sequenceDiagram, pie) |
| Unit tests | 24 (9 refined + 8 v1 + 7 guessing) |
| Integration tests | 5 (3 hangman_refined + 2 guessing_game) |
| CI/CD workflows | 8 (ci, rust-check, gitleaks, markdownlint, pm-evidence, portfolio-ci, bootstrap, docx-to-pdf) |
| Portfolio documents | 10 (README, WEEKS, MIDTERM, FINAL, KEYLOGGER, EVIDENCE_INDEX, SCRIPTS_README, 3 assignments) |
| Scrutiny rounds | 2 (Phase 1: 19 items, Phase 2: 27 items — all resolved) |

## Notes

- Videos and `.docx` files from the source content are **intentionally not** published. Only sanitized transcripts, extracted Rust code, and synthesized summaries are in the public repo.
- Screenshots live under `CC/.../screenshots/` and are tracked via Git LFS. The current evidence strategy uses text-based terminal transcripts.
- Large binary artifacts go through LFS per `.gitattributes`.
- The Rust workspace cannot be compiled locally (Windows toolchain lacks linker). CI handles all build verification.
