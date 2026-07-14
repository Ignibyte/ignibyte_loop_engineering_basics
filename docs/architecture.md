---
type: Architecture
title: Architecture notes
description: The decisions behind the notes service, and the reasons for them that the code cannot tell you.
tags: [architecture, decisions, rust, axum]
timestamp: 2026-07-13T21:23:36-05:00
---

The *why* behind `ignibyte_loop_engineering_basics`. The code can tell a reader
what it does; only this file can tell them what was ruled out, and on what grounds.
Read it before planning a change.

# Shape

- One axum `Router`, built in `lib.rs::app()`, served by a thin `main.rs`.
- Handlers (`routes.rs`) stay thin: parse the request, call into `AppState`, return
  a response. No business logic in handlers.
- `state.rs` is the only place that touches storage. Handlers never reach past it.

# Decisions

| Decision | Why |
|----------|-----|
| **In-memory, file-backed, no database.** | Notes live in a `Vec<Note>` behind a `Mutex` and are mirrored to a JSON file so they survive a restart. A database stays deliberately out of scope until a feature genuinely needs one — a flat file is enough for a single-process demo, and adding Postgres would make this repo a lesson about Postgres. |
| **lib + bin split.** | The router lives in the library so integration tests can drive it with `oneshot`, without opening a socket. `main.rs` only binds a port and serves. |
| **Errors, not panics, in request paths.** | Handlers return a `Result` or a proper status code. The store treats a poisoned lock as unrecoverable; nothing else panics. |
| **Writes are whole-file.** | Every create rewrites the JSON file. Fine at this size; the first thing to revisit if the store ever grows. |

# Citations

[1] [Beginner Loop Engineering](https://ignibyte.com/lab/beginner-loop-engineering-documentation-foundation) - the series this repo was built for.
