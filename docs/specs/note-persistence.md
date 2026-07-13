# Spec: note persistence

Status: **approved** · The feature that travels Parts 3–6 of the Beginner Loop Engineering series.

## Goal

Notes currently live in memory and vanish on restart. Persist them to a JSON file
so they survive a restart. Nothing else changes.

## Approach

- On startup, load notes from a JSON file if it exists; otherwise start empty.
- On create, append to the in-memory list *and* write the file before responding.
- Keep `AppState` (`state.rs`) as the single place that touches storage — no new
  endpoints, no database, no new dependencies (`serde_json` is already present).

## Files to touch

- `src/state.rs` — load-on-init + write-on-add; add a `path` field to `AppState`.
- `src/main.rs` — pass the notes-file path when building state.
- `tests/api.rs` — persistence tests (one per acceptance criterion below).

## Scope fences (what this will NOT do)

- No database. No delete / search / pagination. No new endpoints.
- No file locking or concurrent-writer safety (single process; documented, not solved).
- No migration of an existing store format — there isn't one yet.

## Acceptance criteria (EARS)

- **WHEN** a note is created, the service shall write it to the notes file before returning `201`.
- **WHILE** a notes file exists at startup, the service shall load its notes into memory so `GET /api/notes` returns them.
- **IF** the notes file is missing at startup, **THEN** the service shall start with an empty list (not an error).
- **IF** the notes file exists but is unreadable or malformed, **THEN** the service shall start empty, log a warning, and not crash.
- **IF** a write to the notes file fails, **THEN** the service shall return `500` and leave the in-memory list unchanged.

Each criterion maps to exactly one test in `tests/api.rs`. These are the tests Part 5 will assert — written here, before the code exists, so they can't just mirror the implementation.
