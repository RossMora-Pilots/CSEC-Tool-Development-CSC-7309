#!/usr/bin/env bash
set -euo pipefail

# Optional: sync ROADMAP.md items to GitHub Issues.
# Usage: sync_roadmap.sh <roadmap_path> <owner/repo>
# Requires: GH_TOKEN in env or at ~/.ccpm/secrets/github-token; `gh` CLI optional.

ROADMAP_PATH="${1:-ROADMAP.md}"
GH_REPO="${2:-}"

HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../.." && pwd)"

# Parse roadmap to JSON first
python3 "$ROOT/scripts/roadmap/parse_roadmap.py" "$ROADMAP_PATH" --out "$ROOT/artifacts/roadmap.json"

# Fetch token from local provider if not set
if [ -z "${GH_TOKEN:-}" ] && [ -f "$HOME/.ccpm/secrets/github-token" ]; then
  GH_TOKEN="$(cat "$HOME/.ccpm/secrets/github-token")"
  export GH_TOKEN
fi

if [ -z "${GH_REPO:-}" ]; then echo "SYNC:SKIP_NO_REPO"; exit 0; fi
if ! command -v gh >/dev/null 2>&1; then echo "SYNC:SKIP_NO_GH"; exit 0; fi

python3 "$ROOT/scripts/roadmap/sync_issues.py" --json "$ROOT/artifacts/roadmap.json" --repo "$GH_REPO" || {
  echo "SYNC:ERROR"; exit 0
}
echo "SYNC:OK"
