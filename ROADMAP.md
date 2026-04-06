# ROADMAP — CSEC Tool Development (CSC-7309) — Winter 2025 (Pilot 407)

This pilot creates a public, employer‑facing course portfolio repository for **CSEC Tool Development (CSC‑7309)**, Winter 2025, taught by **Instructor Travis Czech** at Cambrian College (Sudbury, Ontario). It reuses the Pilot 009 template structure and follows the Pilot 008/010 conventions. The course teaches **custom cybersecurity tool development in Rust**, emphasizing memory safety, ownership semantics, and hands-on tool construction.

## Now
- [x] Scaffold repo structure (CC/, docs/, scripts/, portfolio/, .github/workflows/)
- [x] Author root governance files (AGENTS.md, CONTRIBUTING.md, ROADMAP.md)
- [x] Create course-level README with Quick Links and navigation
- [x] Extract and publish Rust code examples (Hangman v1 + refined)
- [x] Author weekly summaries (Weeks 1–6) from lecture transcripts
- [x] Create portfolio writeups (MIDTERM, FINAL, WEEKS summary)
- [x] Configure portfolio/config.json with metrics, skills, references
- [x] Author PM automation scripts (pm.sh, parse_roadmap.py, index_sessions.py)
- [x] Add GitHub Actions workflows (ci, pm-evidence, portfolio-ci, markdownlint, gitleaks)
- [x] Create EVIDENCE_INDEX and SCRIPTS_README

## Next
- [x] Add repo topics/description on GitHub (rust, cybersecurity, tool-development, portfolio, cambrian-college)
- [x] Publish repo to GitHub and set branch protection for `master`
- [x] Run `cargo check` on extracted Rust sources and record passing build
- [x] Add architecture diagram (simple flow: course progression → tools)
- [x] Request instructor attribution/license confirmation (Travis Czech)
- [x] Capture a learning-reflection document for the Rust ownership model

## Later
- [ ] Optional GitHub Pages landing page for the portfolio
- [ ] Expand beyond Week 6 if additional course content becomes available
- [ ] Cross-reference with Pilot 410 (Malware Analysis) for tool-dev continuity
- [x] Add optional Rust test harness for the Hangman examples
- [x] Publish responsible-use advisory for the keylogger lab reference
- [x] Employer-perspective portfolio scrutiny and full remediation (7.8→9.1/10, 19 items resolved)
- [x] Add unit tests to all 3 Cargo projects (24 total: 9 refined + 8 v1 + 7 guessing)
- [x] Add personal work evidence / debugging narratives to all assignments
- [x] Create Cargo workspace, per-project READMEs, and doc comments
- [x] Add CWE vulnerability mappings, effort metrics, and 4 new Mermaid diagrams

## Milestones (Definition of Done)
- [x] Course repo structure finalized and documented
- [x] Evidence artifacts scaffolded (roadmap.json generator, sessions index)
- [x] Portfolio writeups complete with metrics and evidence links
- [x] CI/CD workflows validate markdown, links, scripts, secrets
- [x] Public GitHub repo live with topics, description, and Actions passing
- [ ] Labels normalized and Issues synced (optional); mirrors Pilot 008/010 conventions

## Runbook
- PM loop: `scripts/pm.sh run` (auto‑commit artifacts); `PM_PUSH=1 scripts/pm.sh all`
- Issues (optional): `scripts/pm.sh sync` (requires `gh` + `GH_TOKEN`)
- Skill distribution: `scripts/distribute_skills.sh` (symlink unified-skills to .claude/.codex/.gemini)
- LFS bootstrap: `scripts/setup/setup_git_lfs.sh` (one-time)
