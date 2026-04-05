# Runbook — Pilot 407 (CSEC Tool Development Portfolio)

Daily / weekly operations for this pilot and its public portfolio repo.

## One-Time Setup

```bash
# 1. Clone / navigate into the pilot
cd /mnt/d/pilots/407-Tool-Development

# 2. Distribute unified skills to vendor directories (optional)
bash scripts/distribute_skills.sh

# 3. Bootstrap Git LFS (optional, needed before first binary commit)
bash scripts/setup/setup_git_lfs.sh

# 4. Configure commit template
git config commit.template .gitmessage.txt
```

## PM Loop (parse + index + commit)

```bash
# Run the full PM loop
scripts/pm.sh run

# Or individual steps
scripts/pm.sh parse     # ROADMAP.md -> artifacts/roadmap.json
scripts/pm.sh index     # sessions/*.md -> docs/sessions.md
scripts/pm.sh sync      # (optional) sync to GitHub Issues; requires GH_REPO env var
scripts/pm.sh all       # run everything + auto-commit
```

## Environment Variables

| Variable | Default | Purpose |
|---|---|---|
| `ROADMAP_PATH` | `$ROOT/ROADMAP.md` | Path to roadmap source |
| `GH_REPO` | (unset) | `owner/repo` for Issues sync |
| `PM_COMMIT` | `1` | Auto-commit artifacts (0 to disable) |
| `PM_PUSH` | `0` | Auto-push after commit (1 to enable) |
| `PM_CAPTURE` | `0` | Enable session capture (1 to enable) |
| `SESSIONS_SRC_DIR` | `/mnt/e/sessions-codex` | Source of raw session files |
| `CAPTURE_MAX_LINES` | `120` | Max lines per sanitized excerpt |

## Validating Rust Code Locally

```bash
cd "CC/Winter 2025/CSEC Tool Development - Travis Czech - CSC-7309/scripts/hangman_v1"
cargo check
cargo build
cargo run

cd ../hangman_refined
cargo check
cargo build
cargo run
```

## Pre-Push Checks

```bash
# Markdown lint (needs markdownlint-cli2 installed)
npx markdownlint-cli2 '**/*.md'

# Shell script lint
shellcheck scripts/*.sh scripts/**/*.sh

# Gitleaks (if installed)
gitleaks detect --config .gitleaks.toml
```

## Publishing to GitHub (first time)

```bash
# Create the public repo
gh repo create RossMora/407-tool-development --public \
  --description "Public portfolio — CSEC Tool Development (CSC-7309, Winter 2025, Travis Czech) at Cambrian College"

# Set topics
gh repo edit RossMora/407-tool-development \
  --add-topic rust \
  --add-topic cybersecurity \
  --add-topic tool-development \
  --add-topic portfolio \
  --add-topic cambrian-college \
  --add-topic postgraduate-certificate

# Add the remote and push
git remote add origin git@github.com:RossMora/407-tool-development.git
git push -u origin main
```

## Evidence & Health Checks

After a successful PM run, check:

- `artifacts/roadmap.json` exists and contains current roadmap items
- `docs/sessions.md` reflects `sessions/` contents with SHA256 hashes
- `.github/workflows/` runs are passing on GitHub
- No secrets in the commit history (`gitleaks detect`)
