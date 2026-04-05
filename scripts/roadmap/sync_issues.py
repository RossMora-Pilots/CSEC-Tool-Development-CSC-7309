#!/usr/bin/env python3
"""Sync parsed roadmap items to GitHub Issues.

Best-effort, idempotent: creates a labeled Issue per roadmap item that is not
already tracked. Skips closed Issues. Requires `gh` CLI available on PATH with
GH_TOKEN or equivalent authentication.
"""
import argparse, json, subprocess, sys
from pathlib import Path


def gh(*args, check=True):
    return subprocess.run(["gh", *args], capture_output=True, text=True, check=check)


def list_existing_titles(repo: str):
    """Return the set of Issue titles already in the repo (open + closed)."""
    try:
        r = gh("issue", "list", "--repo", repo, "--state", "all",
               "--limit", "500", "--json", "title", check=False)
        if r.returncode != 0:
            return set()
        data = json.loads(r.stdout or "[]")
        return {i.get("title", "") for i in data}
    except Exception:
        return set()


def create_issue(repo: str, title: str, body: str, labels: list[str]):
    args = ["issue", "create", "--repo", repo, "--title", title, "--body", body]
    for lbl in labels:
        args.extend(["--label", lbl])
    r = gh(*args, check=False)
    return r.returncode == 0


def main():
    p = argparse.ArgumentParser()
    p.add_argument("--json", required=True)
    p.add_argument("--repo", required=True)
    a = p.parse_args()

    data = json.loads(Path(a.json).read_text(encoding="utf-8"))
    existing = list_existing_titles(a.repo)
    sections = data.get("sections", {})

    created = skipped = 0
    for section_path, items in sections.items():
        # Example section_path: "ROADMAP/Now" -> lane = "now"
        lane = (section_path.rsplit("/", 1)[-1] or "unspecified").lower()
        if lane not in {"now", "next", "later"}:
            lane = "unspecified"
        for it in items:
            if it.get("checked"):
                continue  # don't open Issues for done items
            title = f"[roadmap] {it['text']}"
            if title in existing:
                skipped += 1
                continue
            body = (
                f"Auto-generated from ROADMAP.md (line {it.get('line')}).\n\n"
                f"Section: {section_path}\n"
            )
            labels = ["roadmap", f"lane:{lane}", "pilot:407-tool-development"]
            ok = create_issue(a.repo, title, body, labels)
            if ok:
                created += 1
            else:
                skipped += 1
    print(f"SYNC:CREATED={created}:SKIPPED={skipped}")


if __name__ == "__main__":
    main()
