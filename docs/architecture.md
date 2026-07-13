# Architecture notes

The decisions behind `ignibyte_loop_engineering_basics` — the *why* the code
can't tell you. Read this before planning a change.

## Shape

- One axum `Router`, built in `lib.rs::app()`, served by a thin `main.rs`.
- Handlers (`routes.rs`) stay thin: parse the request, call into `AppState`,
  return a response. No business logic in handlers.
- `state.rs` is the only place that touches storage. Handlers never reach past it.

## Decisions

- **In-memory, no database (yet).** Notes live in a `Vec<Note>` behind a
  `Mutex`. Chosen for simplicity while the app is a teaching demo — a database is
  deliberately out of scope until a feature genuinely needs one.
- **lib + bin split.** The router lives in the library so integration tests can
  drive it with `oneshot`, no socket. `main.rs` only binds a port and serves.
- **Errors, not panics, in request paths.** Handlers return a `Result` or a
  proper status code; the store treats a poisoned lock as unrecoverable, and
  nothing else panics.
