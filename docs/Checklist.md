# Checklist — Pilot 407

Near-term work tracker. Complements `ROADMAP.md` with finer-grained steps.

## Done

- [x] Scaffold repo directories (CC/, docs/, scripts/, .github/workflows/, etc.)
- [x] Author root governance files (AGENTS, CONTRIBUTING, ROADMAP, README)
- [x] Author course README with Quick Links and weekly schedule
- [x] Extract Rust Hangman sources into Cargo projects
- [x] Author weekly summary document (Weeks 1-6)
- [x] Author midterm project summary
- [x] Author scoped final-project reflection
- [x] Author EVIDENCE_INDEX and SCRIPTS_README
- [x] Author portfolio/config.json with metrics and skills
- [x] Author PM automation (pm.sh, parse_roadmap.py, index_sessions.py)
- [x] Author 6 GitHub Actions workflows (ci, pm-evidence, portfolio-ci, markdownlint, gitleaks, rust-check)
- [x] Add repo topics/description on GitHub
- [x] Publish repo to GitHub and set branch protection for `master`
- [x] Run `cargo check` on extracted Rust sources and record passing build
- [x] Add architecture diagram (course progression flow)
- [x] Request instructor attribution/license confirmation (Travis Czech)
- [x] Capture learning-reflection document for Rust ownership model
- [x] Add unit tests to all 3 Cargo projects (24 total: 9 refined + 8 v1 + 7 guessing)
- [x] Add personal work evidence / debugging narratives to all assignments
- [x] Create Cargo workspace, per-project READMEs, and doc comments
- [x] Add CWE vulnerability mappings, effort metrics, and 4 new Mermaid diagrams
- [x] Employer-perspective scrutiny Phase 1 (7.8→9.1/10, 19 items)
- [x] Employer-perspective scrutiny Phase 2 (8.4→9.1/10, 27 items)
- [x] Fix metrics mismatch (tests: 9→24, diagrams: 20+→21)
- [x] Add struggle/failure narrative to FINAL_PROJECT
- [x] Add keylogger execution evidence (VM environment, terminal session)
- [x] Add 5 new Mermaid diagrams (gantt, methodology, pie, sequence, repo tree)
- [x] Add employer elevator pitch to root README
- [x] Add system requirements + troubleshooting to SCRIPTS_README
- [x] Fix dead code warning (evaluate_guess behind #[cfg(test)])
- [x] Add cargo-audit + workspace-wide testing + fmt check to CI
- [x] Expand keylogger detection indicators (6 advanced techniques)
- [x] Add institutional approval to keylogger study
- [x] Add course description + prerequisites to Course README
- [x] Add competency checklists to all 3 assignments
- [x] Upgrade ci.yml from trivial to real validation
- [x] Create integration tests for hangman_refined and guessing_game
- [x] Create docs/WORKFLOWS.md for CI/CD pipeline documentation
- [x] Pin Rust version 1.75.0 + edition 2021 in workspace Cargo.toml
- [x] Sync config.json, EVIDENCE_INDEX, and scrutiny report with remediation results

## Next

- [ ] Verify CI passes after remediation commit (rust-check, ci, markdownlint, gitleaks)
- [ ] Push changes to GitHub remote
- [ ] Optional: Add real security tool (port scanner, hash cracker) to reach 9.5/10
- [ ] Optional: GitHub Pages landing page for the portfolio

## Later

- [ ] Expand beyond Week 6 if additional course content becomes available
- [ ] Cross-reference with Pilot 410 (Malware Analysis) for tool-dev continuity
- [ ] Labels normalized and Issues synced (optional)
- [ ] Property-based testing with proptest crate
- [ ] Benchmarks with criterion crate
