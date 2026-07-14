# ignibyte_loop_engineering_basics

A tiny Rust web service — the demo app for the Beginner Loop Engineering series.
It serves a notes app at `/` and a small JSON API. Notes are kept in memory and
mirrored to a JSON file, so they survive a restart. No database.

## Stack
- Rust (2021 edition)
- axum (HTTP framework) on the tokio async runtime
- serde / serde_json for JSON
- Vanilla HTML + CSS + fetch for the page. No framework, no build step.
- Playwright for the browser suite (a dev dependency of the tests, not of the service)

## Layout
- src/lib.rs      — build the router (`app()` / `app_with_state()`); re-exports modules for tests
- src/main.rs     — entry point: bind a port, serve the router
- src/routes.rs   — one handler per endpoint
- src/state.rs    — the note store: in memory, mirrored to a JSON file
- src/model.rs    — the `Note` type, and validation
- static/index.html — the page, embedded into the binary with `include_str!`
- tests/api.rs    — integration tests that drive the router directly, no socket
- e2e/            — Playwright tests that drive a real browser against a real server
- docs/           — the knowledge bundle (see below)
- .claude/        — the hooks that enforce all of this (see `.claude/README.md`)

## Endpoints
- GET  /            — the notes app (HTML)
- GET  /healthz     — liveness check, returns "ok"
- GET  /api/notes   — list all notes as JSON
- POST /api/notes   — add a note (`{ "text": "..." }`). `201` with the note, `400` if the
                      text is empty or whitespace-only, `500` if it could not be saved.

## Commands
- Run:      cargo run              (serves on http://127.0.0.1:3000)
- Gates:    ./check.sh             (fmt, clippy, tests, docs — the one you'll use)
- Browser:  npm run e2e            (first time: npm ci && npm run e2e:install)
- Format:   cargo fmt
- Lint:     cargo clippy --all-targets -- -D warnings

## Conventions
- Errors return a Result or a proper status code — handlers never unwrap untrusted input.
- Every public item and module carries a doc comment.
- Handlers stay thin: read the request, call into state, return a response — no logic inline.
- Validation lives in `model.rs`; storage lives in `state.rs`. Neither leaks into `routes.rs`.
- JSON field names are snake_case.
- Anything a user can see is checked in a browser, not just asserted in a unit test.

## The docs are a knowledge bundle
`docs/` is written in the [Open Knowledge Format][okf] — plain markdown, YAML frontmatter,
a required `type`, and an `index.md` per directory so you can walk it a level at a time
instead of reading all of it. Start at `docs/index.md`.

- `docs/architecture.md` — the decisions and their *why*. **Read this before you plan a change.**
- `docs/specs/` — approved specs. The acceptance criteria in them are the definition of done.
- `docs/log.md` — what changed here, newest first.

Keep them true. The next session starts by trusting them completely, which is exactly what
makes a stale doc more dangerous than a missing one.

## Quality gates
Every change must pass, with zero warnings:

    ./check.sh        # fmt check -> clippy -D warnings -> tests -> docs build
    npm run e2e       # the browser suite

**Mostly green is red.** No partial credit, no "that failure is unrelated."

None of this is on your honour. The hooks in `.claude/` block an edit to `src/` until you
have read the architecture notes, and refuse to let a session end while any gate is red or
any approved spec has a criterion that nothing tests. See `.claude/README.md`.

[okf]: https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md
