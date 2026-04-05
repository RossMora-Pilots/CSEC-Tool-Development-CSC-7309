# Contributing & PM Conventions (Pilot 407 — Tool Development)

- Roadmap‑first: Edit `ROADMAP.md` (Now/Next/Later, Milestones, Runbook)
- Evidence: `scripts/pm.sh run` → `artifacts/roadmap.json`, `docs/sessions.md`, `artifacts/state.json`
- Issues (optional): `scripts/pm.sh sync` (requires `gh` + `GH_TOKEN`)
- Labels: `roadmap`, `lane:now|next|later|unspecified`, `pilot:407-tool-development`
- Secrets: never commit. Fetch via providers; scripts avoid echo; temp files use umask 077.
- Responsible use: security-tool source code (keyloggers, scanners) is published for defensive/educational purposes only and is explicitly labeled as such.

## Commit Messages

Use the structured template from `.gitmessage.txt`:

```
type(scope): concise subject (max 72 chars)

Why
- Motivation

What
- Changes
- Evidence/metrics

Notes
- Links or references
```

Enable with: `git config commit.template .gitmessage.txt`
