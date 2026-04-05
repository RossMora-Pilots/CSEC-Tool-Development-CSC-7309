#!/usr/bin/env python3
"""Extract a capped, sanitized excerpt from a session file.

Redacts common secret-looking patterns (tokens, keys, passwords) and truncates
to --max-lines. Intended for keeping a transparent audit trail in sessions/
without leaking credentials.
"""
import argparse, re
from pathlib import Path


PATTERNS = [
    (re.compile(r"(?i)(gh_?token|github_?token|op_service_account_token)[\"'\s:=]+[A-Za-z0-9_\-]{10,}"), r"\1=REDACTED"),
    (re.compile(r"(?i)(aws_secret_access_key|aws_session_token)[\"'\s:=]+[A-Za-z0-9/+]{10,}"), r"\1=REDACTED"),
    (re.compile(r"(?i)(password|passwd|pwd)[\"'\s:=]+\S{4,}"), r"\1=REDACTED"),
    (re.compile(r"(?i)bearer\s+[A-Za-z0-9_\-\.]+"), "Bearer REDACTED"),
    (re.compile(r"[A-Za-z0-9_\-]{40,}"), "REDACTED_LONG_TOKEN"),
]


def redact(line: str) -> str:
    for pattern, replacement in PATTERNS:
        line = pattern.sub(replacement, line)
    return line


def main():
    p = argparse.ArgumentParser()
    p.add_argument("--in", dest="inputs", nargs="+", required=True)
    p.add_argument("--out", required=True)
    p.add_argument("--max-lines", type=int, default=120)
    p.add_argument("--title", default="Session Excerpt")
    a = p.parse_args()

    buf = []
    for src in a.inputs:
        try:
            content = Path(src).read_text(encoding="utf-8", errors="ignore")
        except Exception:
            continue
        for line in content.splitlines():
            buf.append(redact(line))
            if len(buf) >= a.max_lines:
                break
        if len(buf) >= a.max_lines:
            break

    out = Path(a.out)
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(
        f"# {a.title}\n\n"
        f"_Truncated to {a.max_lines} lines; secrets redacted._\n\n"
        "```\n" + "\n".join(buf) + "\n```\n",
        encoding="utf-8",
    )
    print(f"SANITIZE:OK:{a.out} ({len(buf)} lines)")


if __name__ == "__main__":
    main()
