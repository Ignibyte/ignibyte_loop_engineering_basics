# ignibyte_loop_engineering_basics

A tiny Rust web service — the demo app for the Beginner Loop Engineering series.
It serves a small notes list as JSON and a plain HTML index. In-memory, no database.

## Stack
- Rust (2021 edition)
- axum (HTTP framework) on the tokio async runtime
- serde / serde_json for JSON

## Layout
- src/lib.rs      — build the router (`app()`); re-exports modules for tests
- src/main.rs     — entry point: bind a port, serve `app()`
- src/routes.rs   — one handler per endpoint
- src/state.rs    — the shared in-memory note store
- src/model.rs    — the `Note` type (+ serde derives)
- tests/api.rs    — integration tests that drive the router directly

## Endpoints
- GET  /            — HTML index page
- GET  /healthz     — liveness check, returns "ok"
- GET  /api/notes   — list all notes as JSON
- POST /api/notes   — add a note ({ "text": "..." }); returns it as JSON

## Commands
- Run:     cargo run          (serves on http://127.0.0.1:3000)
- Build:   cargo build
- Test:    cargo test
- Format:  cargo fmt
- Lint:    cargo clippy -- -D warnings

## Conventions
- Errors return a Result or a proper status code — handlers never unwrap untrusted input.
- Every public item and module carries a doc comment.
- Handlers stay thin: read the request, call into state, return a response — no logic inline.
- JSON field names are snake_case.

## Before you plan a change
Read `docs/architecture.md` (the decisions and their *why*) and any relevant
`docs/specs/*.md` first, so your plan builds *with* the codebase, not against it.

## Quality gates
Every change must pass, with zero warnings. One command runs them all, fast to slow:
  ./check.sh          # fmt check -> clippy -D warnings -> tests
CI runs the same script on every push.
