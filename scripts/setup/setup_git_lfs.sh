#!/usr/bin/env bash
set -euo pipefail

# One-time Git LFS bootstrap for this pilot.
# Install git-lfs if missing, ensure .gitattributes has the proper tracking
# patterns, and (optionally) migrate history when MIGRATE=1 is set.

HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../.." && pwd)"

if ! command -v git-lfs >/dev/null 2>&1; then
  if command -v apt-get >/dev/null 2>&1; then
    sudo apt-get update && sudo apt-get install -y git-lfs
  elif command -v brew >/dev/null 2>&1; then
    brew install git-lfs
  else
    echo "Install git-lfs manually: https://git-lfs.com/"
    exit 1
  fi
fi

git lfs install --force

# Ensure patterns are present in .gitattributes
ATTR="$ROOT/.gitattributes"
touch "$ATTR"

ensure_pattern() {
  local pattern="$1"
  if ! grep -qF "$pattern" "$ATTR"; then
    echo "$pattern filter=lfs diff=lfs merge=lfs -text" >> "$ATTR"
  fi
}

ensure_pattern "artifacts/**"
ensure_pattern "images/**"
for ext in docx pptx xlsx zip iso mp4 pdf png jpg; do
  ensure_pattern "*.$ext"
done

if git -C "$ROOT" diff --quiet .gitattributes; then
  echo "LFS:UNCHANGED"
else
  git -C "$ROOT" add .gitattributes
  git -C "$ROOT" -c user.email="codex-bot@local" -c user.name="Codex Bot" \
    commit -m "chore(lfs): ensure LFS tracking for documents/media"
  echo "LFS:COMMITTED"
fi

if [ "${MIGRATE:-0}" = "1" ]; then
  echo "LFS:MIGRATING HISTORY (MIGRATE=1)"
  git -C "$ROOT" lfs migrate import \
    --include="artifacts/**,images/**,*.docx,*.pptx,*.xlsx,*.zip,*.iso,*.mp4,*.pdf,*.png,*.jpg" \
    --include-ref=refs/heads/master --include-ref=refs/heads/main
  echo "LFS:MIGRATED"
fi

echo "LFS:OK"
