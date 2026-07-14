---
type: Spec
title: Note persistence
description: Notes are mirrored to a JSON file so they survive a restart.
status: approved
tags: [spec, storage, persistence]
timestamp: 2026-07-13T21:23:36-05:00
---

The feature that travels Parts 3–6 of the series: planned here, built in Part 4,
proved in Part 5, documented in Part 6.

# Goal

Notes live in memory and vanish on restart. Persist them to a JSON file so they
survive one. Nothing else changes.

# Approach

- On startup, load notes from a JSON file if it exists; otherwise start empty.
- On create, append to the in-memory list *and* write the file before responding.
- Keep `AppState` (`state.rs`) the single place that touches storage — no new
  endpoints, no database, no new dependencies (`serde_json` is already present).

# Files to touch

* `src/state.rs` - load on init, write on add; `AppState` gains a `path`.
* `src/main.rs` - pass the notes-file path when building the state.
* `tests/api.rs` - one test per acceptance criterion below.

# Scope fences

What this deliberately will **not** do:

* No database. No delete, search or pagination. No new endpoints.
* No file locking or concurrent-writer safety — single process; documented, not solved.
* No migration of an existing store format. There isn't one.

# Acceptance criteria

Written before the code existed, so the tests assert the *spec* rather than mirror
the implementation. Each row names the test that proves it, and
`.claude/hooks/spec-done.sh` checks that every one of those tests is real.

| # | Criterion (EARS) | Test | Runner |
|---|------------------|------|--------|
| 1 | WHEN a note is created, the service shall write it to the notes file before returning `201`. | `create_persists_to_file` | rust |
| 2 | WHILE a notes file exists at startup, the service shall load its notes so `GET /api/notes` returns them. | `loads_existing_notes_on_startup` | rust |
| 3 | IF the notes file is missing at startup, THEN the service shall start with an empty list, not an error. | `missing_file_starts_empty` | rust |
| 4 | IF the notes file exists but is unreadable or malformed, THEN the service shall start empty, log a warning, and not crash. | `malformed_file_starts_empty` | rust |
| 5 | IF a write to the notes file fails, THEN the service shall return `500` and leave the in-memory list unchanged. | `write_failure_returns_500_and_keeps_memory` | rust |

# Definition of done

The work is done only when **every criterion above is verified by a passing test**
and the whole suite is green — each one checked, not assumed.

This is not left to good intentions. The `Stop` hook runs `./check.sh`, the browser
suite and `spec-done.sh` before the agent is allowed to finish; while any of them is
red, it goes back to work. An agent asked "are you done?" says yes. Nobody asks it.

# Citations

[1] [Beginner Loop Engineering, Part 3: Plan](https://ignibyte.com/lab/beginner-loop-engineering-plan)
[2] [EARS: Easy Approach to Requirements Syntax](https://alistairmavin.com/ears/)
