# AGENTS.md - 407-Tool-Development

Marker: PROJECT_OK

## Overview

**Purpose:** ROADMAP — CSEC Tool Development (CSC-7309) — Winter 2025 (Pilot 407)

**Scope:** This directory and all subdirectories.

---

## Pre-Flight Awareness Check

Before starting work on this pilot, start your session through the cross-pilot awareness wrapper:

```bash
/mnt/d/pilots/02001-Pilots-Aware-of-Other-Pilots/scripts/start_session.sh "$(basename $PWD)"
```

This runs the pre-flight awareness check, searches 300+ pilots for related work, flags potential duplicates, and writes a local awareness proof stamp used by supported workflows. For deeper searches, use:

```bash
/mnt/d/pilots/02001-Pilots-Aware-of-Other-Pilots/scripts/search_pilots.sh "topic or keyword"
```

---

## Quick Start

```bash
# 1. Check for related existing work
/mnt/d/pilots/02001-Pilots-Aware-of-Other-Pilots/scripts/start_session.sh "$(basename $PWD)"

# 2. Read ROADMAP.md for current tasks
cat ROADMAP.md
```

---

## Safety Rules

- Never commit secrets
- Update state files after completing work
- Create handover record when finishing
